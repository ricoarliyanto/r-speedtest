use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::IntoResponse;

use chrono::{DateTime, Utc};
use sea_orm::Set;

use super::wechat_sdk::{self, amessagetype::*, awechat::*, awechatcrypto::*};
use crate::model::prelude::*;
use crate::model::wechat::entity::wx_users;
use crate::model::wechat::model::mwx_accounts::WxAccountsModel;
use crate::model::wechat::model::mwx_auto_replies::WxAutoRepliesModel;
use crate::model::wechat::model::mwx_messages::{WxMessagesAdd, WxMessagesModel};
use crate::model::wechat::model::mwx_users::{WxUsersAdd, WxUsersModel};
use crate::worker::wx_image_download::{WxImageDownloadMsg, WxImageDownloadWorker};
use crate::worker::AppWorker;

pub async fn official_account(Query(arg): Query<WechatRequest>, body: String) -> impl IntoResponse {
    tracing::info!("收到微信消息: {:?}", body);
    let result = process_wechat_message(&arg, &body).await;

    match result {
        Ok(response) => (
            StatusCode::OK,
            [("Content-Type", "text/xml; charset=utf-8")],
            response,
        ),
        Err(e) => {
            tracing::error!("处理微信消息失败: {:?}", e);
            (
                StatusCode::OK,
                [("Content-Type", "text/plain; charset=utf-8")],
                "Success".to_string(),
            )
        }
    }
}

async fn process_wechat_message(
    arg: &WechatRequest,
    xml_content: &str,
) -> std::result::Result<String, Box<dyn std::error::Error>> {
    let basic_msg = wechat_sdk::parse_basic_wechat_xml(xml_content)?;
    let account = get_account_by_origin_id(&basic_msg.to_user_name).await?;
    verify_request_signature(&account, arg)?;

    let final_message = process_message_by_mode(&account, xml_content).await?;

    let received_time =
        DateTime::from_timestamp(final_message.create_time, 0).unwrap_or_else(Utc::now);

    let wx_messages_add = WxMessagesAdd {
        account_id: account.id,
        msg_id: final_message.msg_id,
        openid: final_message.from_user_name.clone(),
        msg_type: final_message.msg_type.clone(),
        content: final_message.content.clone(),
        media_id: final_message.media_id.clone(),
        pic_url: final_message.pic_url.clone(),
        recognition: final_message.recognition.clone(),
        msg_title: final_message.title.clone(),
        msg_description: final_message.description.clone(),
        link_url: final_message.url.clone(),
        event_type: final_message.event.clone(),
        event_key: final_message.event_key.clone(),
        direction: 1,
        created_at: Some(received_time),
        ..Default::default()
    };
    // 先同步写入消息（需要拿到记录 ID 以便后续更新 pic_url）
    let add_result = WxMessagesModel::add(wx_messages_add).await;
    if let Err(e) = &add_result {
        tracing::error!("写入收到消息失败: {:?}", e);
    }

    // 如果是图片消息，异步下载图片到本地并更新 pic_url
    if final_message.msg_type == "image" {
        if let Some(pic_url) = &final_message.pic_url {
            if !pic_url.is_empty() && pic_url.contains("mmbiz.qpic.cn") {
                if let Ok(record_id_str) = &add_result {
                    // 从 "Successfully added record with id: xxx" 解析 ID
                    if let Some(record_id) = record_id_str
                        .rsplit(':')
                        .next()
                        .and_then(|s| s.trim().parse::<i64>().ok())
                    {
                        let worker_msg = WxImageDownloadMsg {
                            msg_id: record_id,
                            pic_url: pic_url.clone(),
                        };
                        if let Err(e) = WxImageDownloadWorker::enqueue_async(worker_msg).await {
                            tracing::error!("异步下载图片任务入队失败: {:?}", e);
                        }
                    }
                }
            }
        }
    }

    let response = message_handler(final_message.clone(), &account).await;
    if let Ok(resmessage) = wechat_sdk::parse_wechat_xml(&response) {
        // 回复时间 = 收到时间 + 1秒，确保回复晚于收到
        let reply_time = received_time + chrono::Duration::seconds(1);
        let wx_messages_add = WxMessagesAdd {
            account_id: account.id,
            msg_id: resmessage.msg_id,
            openid: resmessage.to_user_name.clone(),
            msg_type: resmessage.msg_type.clone(),
            content: resmessage.content.clone(),
            direction: 0,
            created_at: Some(reply_time),
            ..Default::default()
        };
        // 后台写入回复消息，不阻塞响应
        let db_add2 = WxMessagesModel::add(wx_messages_add);
        tokio::spawn(async move {
            if let Err(e) = db_add2.await {
                tracing::error!("写入回复消息失败: {:?}", e);
            }
        });
    }

    Ok(response)
}

fn verify_request_signature(
    account: &WxAccountsModel,
    arg: &WechatRequest,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let token = account.token.as_deref().unwrap_or("");

    if !wechat_sdk::verify_signature(token, &arg.timestamp, &arg.nonce, &arg.signature) {
        return Err("签名验证失败".into());
    }

    Ok(())
}

async fn process_message_by_mode(
    account: &WxAccountsModel,
    xml_content: &str,
) -> std::result::Result<WechatMessage, Box<dyn std::error::Error>> {
    match account.message_mode {
        0 => {
            // 明文模式
            wechat_sdk::parse_wechat_xml(xml_content).map_err(|e| e.into())
        }
        1 => {
            // 兼容模式
            wechat_sdk::parse_wechat_xml(xml_content).map_err(|e| e.into())
        }
        _ => {
            // 加密模式
            process_encrypted_message(account, xml_content).await
        }
    }
}

async fn process_encrypted_message(
    account: &WxAccountsModel,
    xml_content: &str,
) -> std::result::Result<WechatMessage, Box<dyn std::error::Error>> {
    let encrypted_msg = wechat_sdk::parse_wechat_encry_xml(xml_content)?;

    let encrypt_data = encrypted_msg.encrypt.ok_or("加密数据为空")?;

    let token = account.token.as_deref().unwrap_or_default();
    let aes_key = account.encoding_aes_key.as_deref().unwrap_or_default();
    let app_id = &account.app_id;

    let wechat_crypto = WechatCrypto::new(token.to_owned(), aes_key.to_owned(), app_id.to_owned())?;

    let decrypted_xml = wechat_crypto.decrypt_message(&encrypt_data)?;
    tracing::info!("解密成功: {}", decrypted_xml);

    wechat_sdk::parse_wechat_xml(&decrypted_xml).map_err(|e| e.into())
}

async fn get_account_by_origin_id(
    to_user_name: &str,
) -> std::result::Result<WxAccountsModel, Box<dyn std::error::Error>> {
    // 先按 original_id 查找（微信回调中的 ToUserName 通常是原始ID gh_xxx）
    match WxAccountsModel::find_by_origin_id(to_user_name).await {
        Ok(Some(account)) => return Ok(account),
        Ok(None) => { /* 继续尝试其他方式 */ }
        Err(e) => {
            tracing::error!("查询账户失败: {:?}", e);
            return Err("数据库查询失败".into());
        }
    }

    // 再按 wechat_id 查找
    let db = DB().await;
    match crate::model::wechat::entity::wx_accounts::Entity::find()
        .filter(crate::model::wechat::entity::wx_accounts::Column::WechatId.eq(to_user_name))
        .one(db)
        .await
    {
        Ok(Some(account)) => return Ok(account),
        Ok(None) => { /* 继续 */ }
        Err(e) => {
            tracing::error!("查询账户失败: {:?}", e);
            return Err("数据库查询失败".into());
        }
    }

    // 最后按 app_id 查找（微信回调中 ToUserName 也可能是 app_id）
    let db = DB().await;
    match crate::model::wechat::entity::wx_accounts::Entity::find()
        .filter(crate::model::wechat::entity::wx_accounts::Column::AppId.eq(to_user_name))
        .one(db)
        .await
    {
        Ok(Some(account)) => Ok(account),
        Ok(None) => Err("账户不存在".into()),
        Err(e) => {
            tracing::error!("查询账户失败: {:?}", e);
            Err("数据库查询失败".into())
        }
    }
}

async fn message_handler(msg: WechatMessage, account: &WxAccountsModel) -> String {
    let now = Utc::now();
    let account_id = account.id;

    // 更新/创建用户
    let user = WxUsersModel::find_by_openid(account_id, &msg.from_user_name)
        .await
        .ok()
        .flatten();

    // 加载该账户所有启用的自动回复规则（已按优先级降序排列）
    let auto_replies = WxAutoRepliesModel::find_enabled_by_account(account_id)
        .await
        .unwrap_or_default();

    let msg_type = MessageType::from(msg.msg_type.as_str());
    match msg_type {
        MessageType::Text => {
            // 更新用户最后互动时间
            update_user_interact(&user, now).await;

            let content = match msg.content.as_ref() {
                Some(content) => content.clone(),
                None => return wechat_sdk::reply_wechat_xml(&msg, "谢谢您的关注".to_string()),
            };

            // 1. 尝试匹配关键词回复 (reply_type=2)
            for rule in &auto_replies {
                if rule.reply_type != 2 {
                    continue;
                }
                if keyword_match(&content, rule.keyword.as_deref(), rule.match_type) {
                    tracing::info!(
                        "关键词匹配命中: keyword={:?}, content={}",
                        rule.keyword,
                        content
                    );
                    return build_reply_xml(&msg, rule);
                }
            }

            // 2. 尝试默认回复 (reply_type=3)
            for rule in &auto_replies {
                if rule.reply_type == 3 {
                    tracing::info!("使用默认回复规则: id={}", rule.id);
                    return build_reply_xml(&msg, rule);
                }
            }

            // 3. 无匹配规则，返回默认
            wechat_sdk::reply_wechat_xml(&msg, "谢谢您的关注".to_string())
        }
        MessageType::Image => {
            update_user_interact(&user, now).await;
            wechat_sdk::reply_wechat_xml(&msg, "谢谢您的关注".to_string())
        }
        MessageType::Event => {
            let event_str = match msg.event.as_ref() {
                Some(event) => event.clone(),
                None => return wechat_sdk::reply_wechat_xml(&msg, "谢谢您的关注".to_string()),
            };

            let event_type = EventType::from(event_str.as_str());
            match event_type {
                EventType::Subscribe => {
                    // 先尝试关注回复规则 (reply_type=1)
                    for rule in &auto_replies {
                        if rule.reply_type == 1 {
                            tracing::info!("使用关注回复规则: id={}", rule.id);
                            let resp = build_reply_xml(&msg, rule);
                            // 创建或更新微信用户
                            upsert_user(user, account_id, &msg.from_user_name, now).await;
                            return resp;
                        }
                    }

                    // 没有配置关注回复，用默认欢迎语
                    let resp = wechat_sdk::reply_wechat_xml(&msg, "欢迎关注".to_string());
                    upsert_user(user, account_id, &msg.from_user_name, now).await;
                    resp
                }
                EventType::Click => {
                    // 点击事件：尝试匹配 event_key 作为关键词
                    if let Some(event_key) = &msg.event_key {
                        for rule in &auto_replies {
                            if rule.reply_type != 2 {
                                continue;
                            }
                            if keyword_match(event_key, rule.keyword.as_deref(), rule.match_type) {
                                return build_reply_xml(&msg, rule);
                            }
                        }
                    }
                    wechat_sdk::reply_wechat_xml(&msg, "点击事件".to_string())
                }
                EventType::Unsubscribe => {
                    if let Some(user) = &user {
                        let mut active: wx_users::ActiveModel = user.clone().into();
                        active.subscribe_status = Set(Some(0));
                        active.unsubscribe_time = Set(Some(now));
                        let _ = active.update(DB().await).await;
                    }
                    wechat_sdk::reply_wechat_xml(&msg, "谢谢您的关注".to_string())
                }
                _ => wechat_sdk::reply_wechat_xml(&msg, "谢谢您的关注".to_string()),
            }
        }
        _ => wechat_sdk::reply_wechat_xml(&msg, "谢谢您的关注".to_string()),
    }
}

/// 关键词匹配：match_type 1=完全匹配, 2=包含匹配
fn keyword_match(content: &str, keyword: Option<&str>, match_type: Option<i8>) -> bool {
    let kw = match keyword {
        Some(kw) if !kw.is_empty() => kw,
        _ => return false,
    };
    match match_type {
        Some(1) => content == kw,
        Some(2) | None => content.contains(kw),
        _ => content.contains(kw),
    }
}

/// 根据自动回复规则生成对应类型的回复 XML
fn build_reply_xml(msg: &WechatMessage, rule: &WxAutoRepliesModel) -> String {
    match rule.message_type.as_str() {
        "image" => {
            let media_id = rule.media_id.as_deref().unwrap_or_default();
            wechat_sdk::reply_wechat_image_xml(msg, media_id)
        }
        "news" => {
            let title = rule.title.as_deref().unwrap_or_default();
            let description = rule.description.as_deref().unwrap_or_default();
            let pic_url = rule.pic_url.as_deref().unwrap_or_default();
            let url = rule.url.as_deref().unwrap_or_default();
            wechat_sdk::reply_wechat_news_xml(msg, title, description, pic_url, url)
        }
        "music" => {
            let title = rule.title.as_deref().unwrap_or_default();
            let description = rule.description.as_deref().unwrap_or_default();
            let music_url = rule.music_url.as_deref().unwrap_or_default();
            let hq_music_url = rule.hq_music_url.as_deref().unwrap_or_default();
            let thumb_media_id = rule.thumb_media_id.as_deref().unwrap_or_default();
            wechat_sdk::reply_wechat_music_xml(
                msg,
                title,
                description,
                music_url,
                hq_music_url,
                thumb_media_id,
            )
        }
        _ => {
            // text 及其他类型默认按文本回复
            let content = rule.content.as_deref().unwrap_or("谢谢您的关注");
            wechat_sdk::reply_wechat_xml(msg, content.to_string())
        }
    }
}

/// 更新用户最后互动时间
async fn update_user_interact(user: &Option<WxUsersModel>, now: DateTime<Utc>) {
    if let Some(user) = user {
        let mut active: wx_users::ActiveModel = user.clone().into();
        active.last_interact_time = Set(Some(now));
        active.message_count = Set(Some(user.message_count.unwrap_or(0) + 1));
        let _ = active.update(DB().await).await;
    }
}

/// 创建或更新微信用户（关注时）
async fn upsert_user(
    user: Option<WxUsersModel>,
    account_id: i64,
    openid: &str,
    now: DateTime<Utc>,
) {
    match user {
        Some(user) => {
            let mut active: wx_users::ActiveModel = user.into();
            active.subscribe_status = Set(Some(1));
            active.subscribe_time = Set(Some(now));
            active.unsubscribe_time = Set(None);
            let _ = active.update(DB().await).await;
        }
        None => {
            let _ = WxUsersModel::add(WxUsersAdd {
                account_id,
                openid: openid.to_string(),
                subscribe_status: Some(1),
                subscribe_time: Some(now),
                ..Default::default()
            })
            .await;
        }
    }
}

pub async fn official_account_get(VQuery(arg): VQuery<WeChatVerifyQuery>) -> impl IntoResponse {
    tracing::info!("get arg: {:?}", arg);
    (StatusCode::OK, arg.echostr)
}
