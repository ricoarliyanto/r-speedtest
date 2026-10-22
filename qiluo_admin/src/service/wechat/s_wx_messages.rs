use super::wechat_sdk::wechatclient::WeChatClient;
use crate::model::wechat::args::awx_messages::{WxConversationQuery, WxMessageReply, WxMessageStreamQuery};
use crate::model::wechat::model::mwx_accounts::WxAccountsModel;
use crate::model::wechat::model::mwx_messages::{
    WxMessagesAdd, WxMessagesEdit, WxMessagesModel, WxMessagesSearch, WxMessagesDel,
};
use crate::service::prelude::*;
use axum::response::sse::{Event, Sse};
use chrono::Utc;
use futures::stream::{self, Stream};
use std::time::Duration;
use axum::body::Body;
use axum::extract::Query;
use axum::http::{HeaderMap, HeaderValue, header};
use axum::response::IntoResponse;
use std::collections::HashMap;

/// SSE: 实时消息流
pub async fn message_stream(
    VQuery(arg): VQuery<WxMessageStreamQuery>,
) -> Sse<impl Stream<Item = std::result::Result<Event, crate::common::error::Error>>> {
    let account_id = arg.account_id;
    let last_id = arg.last_id.unwrap_or(0);

    let stream = stream::unfold(
        (account_id, last_id, false),
        move |(aid, lid, has_data)| async move {
            tokio::time::sleep(Duration::from_secs(3)).await;
            let db = DB().await;

            // 查新消息
            use sea_orm::{EntityTrait, QueryOrder};
            use crate::model::wechat::entity::wx_messages;
            let msgs = wx_messages::Entity::find()
                .filter(wx_messages::Column::AccountId.eq(aid))
                .filter(wx_messages::Column::Id.gt(lid))
                .order_by_asc(wx_messages::Column::Id)
                .all(db)
                .await
                .ok()
                .unwrap_or_default();

            if msgs.is_empty() {
                if !has_data {
                    return Some((Ok(Event::default().comment("waiting")), (aid, lid, false)));
                }
                return Some((Ok(Event::default().comment("keep-alive")), (aid, lid, true)));
            }

            let max_id = msgs.iter().map(|m| m.id).max().unwrap_or(lid);
            let json = serde_json::to_string(&msgs).unwrap_or_else(|_| "[]".to_string());
            let event = Event::default()
                .data(json)
                .event("new_messages");
            Some((Ok(event), (aid, max_id, true)))
        },
    );

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(10))
            .text("keep-alive-text"),
    )
}

pub async fn list_tree(
    VQuery(arg): VQuery<PageParams>,
    VQuery(search): VQuery<WxMessagesSearch>,
) -> impl IntoResponse {
    let rlist = WxMessagesModel::list(arg, search).await;
    ApiResponse::from_result(rlist)
}
pub async fn edit(VJson(arg): VJson<WxMessagesEdit>) -> impl IntoResponse {
    let r = WxMessagesModel::edit(arg).await;
    ApiResponse::from_result(r)
}
pub async fn add(VJson(arg): VJson<WxMessagesAdd>) -> impl IntoResponse {
    let r = WxMessagesModel::add(arg).await;
    ApiResponse::from_result(r)
}
pub async fn delete(VQuery(arg): VQuery<WxMessagesDel>) -> impl IntoResponse {
    let r = WxMessagesModel::del(arg).await;
    ApiResponse::from_result(r)
}

/// 获取会话消息列表
pub async fn conversation(VQuery(arg): VQuery<WxConversationQuery>) -> impl IntoResponse {
    let r = WxMessagesModel::get_conversation(arg.account_id, &arg.openid).await;
    ApiResponse::from_result(r)
}

/// 手动回复消息（通过微信客服消息接口）
pub async fn reply_message(VJson(arg): VJson<WxMessageReply>) -> impl IntoResponse {
    let account = WxAccountsModel::find_by_id(arg.account_id).await;
    match account {
        Ok(Some(mp)) => {
            let client = WeChatClient::new(mp.app_id, mp.app_secret);
            let result = match arg.msg_type.as_str() {
                "text" => {
                    let content = match &arg.content {
                        Some(c) => c.clone(),
                        None => return ApiResponse::bad_request("文本消息内容不能为空"),
                    };
                    client.send_custom_text_message(&arg.openid, &content).await
                }
                "image" => {
                    let media_id = match &arg.media_id {
                        Some(m) => m.clone(),
                        None => return ApiResponse::bad_request("图片消息素材ID不能为空"),
                    };
                    client.send_custom_image_message(&arg.openid, &media_id).await
                }
                "link" => {
                    let title = match &arg.title {
                        Some(t) => t.clone(),
                        None => return ApiResponse::bad_request("链接消息标题不能为空"),
                    };
                    let url = match &arg.url {
                        Some(u) => u.clone(),
                        None => return ApiResponse::bad_request("链接消息URL不能为空"),
                    };
                    client.send_custom_link_message(
                        &arg.openid,
                        &title,
                        arg.description.as_deref().unwrap_or(""),
                        &url,
                        arg.thumb_url.as_deref().unwrap_or(""),
                    ).await
                }
                _ => return ApiResponse::bad_request("不支持的消息类型"),
            };

            match result {
                Ok(_) => {
                    // 保存发送记录到数据库
                    let add_arg = WxMessagesAdd {
                        account_id: arg.account_id,
                        openid: arg.openid.clone(),
                        msg_id: None,
                        msg_type: arg.msg_type.clone(),
                        direction: 2, // 发送
                        content: arg.content.clone(),
                        media_id: arg.media_id.clone(),
                        pic_url: None,
                        voice_format: None,
                        recognition: None,
                        thumb_media_id: None,
                        msg_title: arg.title.clone(),
                        msg_description: arg.description.clone(),
                        link_url: arg.url.clone(),
                        event_type: None,
                        event_key: None,
                        reply_msg_id: None,
                        is_auto_reply: Some(0),
                        created_at: Some(Utc::now()),
                    };
                    let _ = WxMessagesModel::add(add_arg).await;
                    ApiResponse::ok("消息发送成功")
                }
                Err(e) => ApiResponse::internal_server_error(e.to_string()),
            }
        }
        _ => ApiResponse::not_found("公众号不存在"),
    }
}

/// 代理微信图片（绕过防盗链和 HTTPS 混合内容限制）
/// 前端通过 /api/wechat/wxmessages/proxy_image?url=xxx 访问
pub async fn proxy_image(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let url = match params.get("url") {
        Some(u) if !u.is_empty() => u.clone(),
        _ => {
            return (
                axum::http::StatusCode::BAD_REQUEST,
                [(header::CONTENT_TYPE, "text/plain")],
                "Missing url parameter".to_string(),
            )
                .into_response();
        }
    };

    // 只允许代理微信图片域名，防止被滥用为开放代理
    if !url.starts_with("http://mmbiz.qpic.cn/")
        && !url.starts_with("https://mmbiz.qpic.cn/")
    {
        return (
            axum::http::StatusCode::FORBIDDEN,
            [(header::CONTENT_TYPE, "text/plain")],
            "Only mmbiz.qpic.cn URLs are allowed".to_string(),
        )
            .into_response();
    }

    let client = reqwest::Client::new();
    match client.get(&url).send().await {
        Ok(resp) => {
            let status = resp.status();
            let content_type = resp
                .headers()
                .get("content-type")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("image/jpeg")
                .to_string();

            let body = match resp.bytes().await {
                Ok(b) => b,
                Err(_) => {
                    return (
                        axum::http::StatusCode::BAD_GATEWAY,
                        [(header::CONTENT_TYPE, "text/plain")],
                        "Failed to read image data".to_string(),
                    )
                        .into_response();
                }
            };

            let mut headers = HeaderMap::new();
            headers.insert(header::CONTENT_TYPE, HeaderValue::from_str(&content_type).unwrap_or(HeaderValue::from_static("image/jpeg")));
            headers.insert(header::CACHE_CONTROL, HeaderValue::from_static("public, max-age=86400"));
            headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"));

            (status, headers, Body::from(body)).into_response()
        }
        Err(_) => (
            axum::http::StatusCode::BAD_GATEWAY,
            [(header::CONTENT_TYPE, "text/plain")],
            "Failed to fetch image from WeChat".to_string(),
        )
            .into_response(),
    }
}
