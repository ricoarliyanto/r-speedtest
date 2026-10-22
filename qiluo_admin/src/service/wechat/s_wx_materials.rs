use super::wechat_sdk::{material::amaterial::*, wechatclient::WeChatClient};
use crate::model::wechat::model::{
    mwx_accounts::WxAccountsModel,
    mwx_materials::{
        WxMaterialsAdd, WxMaterialsDel, WxMaterialsEdit, WxMaterialsModel, WxMaterialsSearch,
        SyncMaterialsReq, MaterialCountReq, DeleteRemoteMediaReq, UploadTempMediaReq,
        UploadPermanentMediaReq, UploadNewsReq,
    },
};
use crate::model::prelude::*;
use crate::service::prelude::*;

/// 获取素材列表
pub async fn list(
    VQuery(arg): VQuery<PageParams>,
    VQuery(search): VQuery<WxMaterialsSearch>,
) -> impl IntoResponse {
    let rlist = WxMaterialsModel::list(arg, search).await;
    ApiResponse::from_result(rlist)
}

/// 添加素材
pub async fn add(VJson(arg): VJson<WxMaterialsAdd>) -> impl IntoResponse {
    let r = WxMaterialsModel::add(arg).await;
    ApiResponse::from_result(r)
}

/// 编辑素材
pub async fn edit(VJson(arg): VJson<WxMaterialsEdit>) -> impl IntoResponse {
    let r = WxMaterialsModel::edit(arg).await;
    ApiResponse::from_result(r)
}

/// 删除素材
pub async fn delete(VQuery(arg): VQuery<WxMaterialsDel>) -> impl IntoResponse {
    let r = WxMaterialsModel::del(arg).await;
    ApiResponse::from_result(r)
}

/// 上传临时素材到微信服务器
pub async fn upload_temp_media(VJson(arg): VJson<UploadTempMediaReq>) -> impl IntoResponse {
    let account = WxAccountsModel::find_by_id(arg.account_id).await;
    match account {
        Ok(Some(mp)) => {
            // 需要从本地路径读取文件上传
            let local_path = match find_local_file(arg.account_id).await {
                Some(path) => path,
                None => return ApiResponse::bad_request("未找到待上传的本地文件，请先通过Multipart上传文件".to_string()),
            };

            let client = WeChatClient::new(mp.app_id, mp.app_secret);
            let file_name = std::path::Path::new(&local_path)
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default();

            let result = client
                .upload_temp_media(&arg.media_type, &local_path, &file_name)
                .await;
            match result {
                Ok(resp) => {
                    // 保存到本地数据库
                    let now = chrono::Utc::now();
                    let add_arg = WxMaterialsAdd {
                        account_id: arg.account_id,
                        media_type: resp.media_type.clone(),
                        media_id: Some(resp.media_id.clone()),
                        name: Some(file_name),
                        url: None,
                        local_path: Some(local_path),
                        file_size: None,
                        content_type: None,
                        width: None,
                        height: None,
                        duration: None,
                        description: None,
                        title: None,
                        introduction: None,
                        thumb_media_id: None,
                        thumb_url: None,
                        content_source_url: None,
                        digest: None,
                        author: None,
                        content: None,
                        news_items: None,
                        is_permanent: Some(0), // 临时素材
                        sync_status: Some(1),
                        synced_at: Some(now),
                        created_at: None,
                        updated_at: None,
                    };
                    let _ = WxMaterialsModel::add(add_arg).await;
                    ApiResponse::ok(resp)
                }
                Err(e) => ApiResponse::internal_server_error(e.to_string()),
            }
        }
        _ => ApiResponse::not_found("公众号不存在"),
    }
}

/// 上传永久素材到微信服务器
pub async fn upload_permanent_media(VJson(arg): VJson<UploadPermanentMediaReq>) -> impl IntoResponse {
    let account = WxAccountsModel::find_by_id(arg.account_id).await;
    match account {
        Ok(Some(mp)) => {
            let local_path = match find_local_file(arg.account_id).await {
                Some(path) => path,
                None => return ApiResponse::bad_request("未找到待上传的本地文件，请先通过Multipart上传文件".to_string()),
            };

            let client = WeChatClient::new(mp.app_id, mp.app_secret);
            let file_name = std::path::Path::new(&local_path)
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default();

            let result = if arg.media_type == "video" {
                let title = arg.title.clone().unwrap_or_default();
                let introduction = arg.introduction.clone().unwrap_or_default();
                client
                    .upload_permanent_video(&local_path, &file_name, &title, &introduction)
                    .await
            } else {
                client
                    .upload_permanent_media(&arg.media_type, &local_path, &file_name)
                    .await
            };

            match result {
                Ok(resp) => {
                    let now = chrono::Utc::now();
                    let add_arg = WxMaterialsAdd {
                        account_id: arg.account_id,
                        media_type: arg.media_type.clone(),
                        media_id: Some(resp.media_id.clone()),
                        name: Some(file_name),
                        url: resp.url.clone(),
                        local_path: Some(local_path),
                        file_size: None,
                        content_type: None,
                        width: None,
                        height: None,
                        duration: None,
                        description: None,
                        title: arg.title.clone(),
                        introduction: arg.introduction.clone(),
                        thumb_media_id: None,
                        thumb_url: None,
                        content_source_url: None,
                        digest: None,
                        author: None,
                        content: None,
                        news_items: None,
                        is_permanent: Some(1), // 永久素材
                        sync_status: Some(1),
                        synced_at: Some(now),
                        created_at: None,
                        updated_at: None,
                    };
                    let _ = WxMaterialsModel::add(add_arg).await;
                    ApiResponse::ok(resp)
                }
                Err(e) => ApiResponse::internal_server_error(e.to_string()),
            }
        }
        _ => ApiResponse::not_found("公众号不存在"),
    }
}

/// 上传永久图文素材
pub async fn upload_news(VJson(arg): VJson<UploadNewsReq>) -> impl IntoResponse {
    let account = WxAccountsModel::find_by_id(arg.account_id).await;
    match account {
        Ok(Some(mp)) => {
            let client = WeChatClient::new(mp.app_id, mp.app_secret);

            // 构建图文素材请求
            let articles: Vec<NewsArticle> = arg.articles.iter().map(|item| NewsArticle {
                thumb_media_id: item.thumb_media_id.clone(),
                author: item.author.clone(),
                title: item.title.clone(),
                content: item.content.clone(),
                content_source_url: item.content_source_url.clone(),
                digest: item.digest.clone(),
                show_cover_pic: item.show_cover_pic.map(|v| v as i64),
                need_open_comment: item.need_open_comment.map(|v| v as i64),
                only_fans_can_comment: item.only_fans_can_comment.map(|v| v as i64),
            }).collect();

            let result = client.upload_news(&articles).await;
            match result {
                Ok(resp) => {
                    let now = chrono::Utc::now();
                    let add_arg = WxMaterialsAdd {
                        account_id: arg.account_id,
                        media_type: "news".to_string(),
                        media_id: Some(resp.media_id.clone()),
                        name: None,
                        url: None,
                        local_path: None,
                        file_size: None,
                        content_type: None,
                        width: None,
                        height: None,
                        duration: None,
                        description: None,
                        title: Some(arg.articles.first().map(|a| a.title.clone()).unwrap_or_default()),
                        introduction: None,
                        thumb_media_id: Some(arg.articles.first().map(|a| a.thumb_media_id.clone()).unwrap_or_default()),
                        thumb_url: None,
                        content_source_url: arg.articles.first().and_then(|a| a.content_source_url.clone()),
                        digest: arg.articles.first().and_then(|a| a.digest.clone()),
                        author: arg.articles.first().and_then(|a| a.author.clone()),
                        content: Some(serde_json::to_string(&arg.articles).unwrap_or_default()),
                        news_items: Some(serde_json::to_string(&arg.articles).unwrap_or_default()),
                        is_permanent: Some(1),
                        sync_status: Some(1),
                        synced_at: Some(now),
                        created_at: None,
                        updated_at: None,
                    };
                    let _ = WxMaterialsModel::add(add_arg).await;
                    ApiResponse::ok(resp)
                }
                Err(e) => ApiResponse::internal_server_error(e.to_string()),
            }
        }
        _ => ApiResponse::not_found("公众号不存在"),
    }
}

/// 从微信同步素材到本地
pub async fn sync_materials(VJson(arg): VJson<SyncMaterialsReq>) -> impl IntoResponse {
    let account = WxAccountsModel::find_by_id(arg.account_id).await;
    match account {
        Ok(Some(mp)) => {
            let client = WeChatClient::new(mp.app_id, mp.app_secret);
            let offset = arg.offset.unwrap_or(0);
            let count = arg.count.unwrap_or(20).min(20);

            let result = client
                .batch_get_material(&arg.media_type, offset, count)
                .await;
            match result {
                Ok(resp) => {
                    let now = chrono::Utc::now();
                    let mut items_to_add: Vec<WxMaterialsAdd> = Vec::new();

                    for item in &resp.item {
                        // 检查是否已存在
                        if let Ok(Some(_)) = WxMaterialsModel::find_by_media_id(arg.account_id, &item.media_id).await {
                            continue; // 跳过已存在的
                        }

                        let (title, author, digest, content, content_source_url, thumb_media_id, url) =
                            parse_material_item(item, &arg.media_type);

                        let add_arg = WxMaterialsAdd {
                            account_id: arg.account_id,
                            media_type: arg.media_type.clone(),
                            media_id: Some(item.media_id.clone()),
                            name: item.name.clone(),
                            url,
                            local_path: None,
                            file_size: None,
                            content_type: None,
                            width: None,
                            height: None,
                            duration: None,
                            description: None,
                            title,
                            introduction: None,
                            thumb_media_id,
                            thumb_url: None,
                            content_source_url,
                            digest,
                            author,
                            content,
                            news_items: None,
                            is_permanent: Some(1), // 同步的都是永久素材
                            sync_status: Some(1),
                            synced_at: Some(now),
                            created_at: None,
                            updated_at: None,
                        };
                        items_to_add.push(add_arg);
                    }

                    let added_count = items_to_add.len();
                    if !items_to_add.is_empty() {
                        match WxMaterialsModel::batch_add(items_to_add).await {
                            Ok(_) => ApiResponse::ok(format!(
                                "同步成功，共 {} 条素材，新增 {} 条（总计 {} 条）",
                                resp.item_count, added_count, resp.total_count
                            )),
                            Err(e) => ApiResponse::internal_server_error(e.to_string()),
                        }
                    } else {
                        ApiResponse::ok(format!(
                            "同步完成，共 {} 条素材，全部已存在无需新增（总计 {} 条）",
                            resp.item_count, resp.total_count
                        ))
                    }
                }
                Err(e) => ApiResponse::internal_server_error(e.to_string()),
            }
        }
        _ => ApiResponse::not_found("公众号不存在"),
    }
}

/// 获取微信素材计数
pub async fn material_count(VJson(arg): VJson<MaterialCountReq>) -> impl IntoResponse {
    let account = WxAccountsModel::find_by_id(arg.account_id).await;
    match account {
        Ok(Some(mp)) => {
            let client = WeChatClient::new(mp.app_id, mp.app_secret);
            let result = client.get_material_count().await;
            match result {
                Ok(resp) => ApiResponse::ok(resp),
                Err(e) => ApiResponse::internal_server_error(e.to_string()),
            }
        }
        _ => ApiResponse::not_found("公众号不存在"),
    }
}

/// 删除微信服务器上的永久素材
pub async fn delete_remote_media(VJson(arg): VJson<DeleteRemoteMediaReq>) -> impl IntoResponse {
    let account = WxAccountsModel::find_by_id(arg.account_id).await;
    match account {
        Ok(Some(mp)) => {
            let client = WeChatClient::new(mp.app_id, mp.app_secret);
            let result = client.delete_material(&arg.media_id).await;
            match result {
                Ok(msg) => {
                    // 同步删除本地记录
                    if let Ok(Some(local)) = WxMaterialsModel::find_by_media_id(arg.account_id, &arg.media_id).await {
                        let _ = WxMaterialsModel::del(WxMaterialsDel { id: local.id }).await;
                    }
                    ApiResponse::ok(&msg)
                }
                Err(e) => ApiResponse::internal_server_error(e.to_string()),
            }
        }
        _ => ApiResponse::not_found("公众号不存在"),
    }
}

/// Multipart上传文件到服务器本地
pub async fn upload_file(mut multipart: Multipart) -> impl IntoResponse {
    let mut file_path = String::new();
    let mut file_name = String::new();
    let mut account_id: i64 = 0;

    while let Some(field) = multipart.next_field().await.unwrap_or(None) {
        let field_name = field.name().unwrap_or("").to_string();

        match field_name.as_str() {
            "file" => {
                file_name = field.file_name().unwrap_or("upload").to_string();
                let data = field.bytes().await.unwrap_or_default();

                // 生成保存路径
                let upload_dir = "uploads/wx_materials".to_string();
                let _ = std::fs::create_dir_all(&upload_dir);
                let save_path = format!("{}/{}_{}", upload_dir, chrono::Utc::now().timestamp_millis(), file_name);

                std::fs::write(&save_path, &data).unwrap_or_else(|e| {
                    tracing::error!("保存上传文件失败: {}", e);
                });
                file_path = save_path;
            }
            "account_id" => {
                let data = field.text().await.unwrap_or_default();
                account_id = data.parse().unwrap_or(0);
            }
            _ => {}
        }
    }

    if file_path.is_empty() {
        return ApiResponse::bad_request("未找到上传文件".to_string());
    }

    ApiResponse::ok(serde_json::json!({
        "file_path": file_path,
        "file_name": file_name,
        "account_id": account_id,
    }))
}

/// 辅助函数：查找本地待上传文件
async fn find_local_file(_account_id: i64) -> Option<String> {
    // 此处可根据业务逻辑查找临时文件路径
    // 上传文件流程: 先调用 upload_file 获取 file_path，再将 file_path 传给上传接口
    None
}

/// 解析素材条目，提取不同类型素材的字段
fn parse_material_item(
    item: &MaterialItem,
    media_type: &str,
) -> (Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>) {
    match media_type {
        "news" => {
            // 图文素材 - 从 content 或 news_item 中提取
            let news_item = item.content.as_ref()
                .and_then(|c| c.news_item.first().cloned())
                .or_else(|| item.news_item.as_ref().and_then(|items| items.first().cloned()));

            (
                news_item.as_ref().map(|n| n.title.clone()),
                news_item.as_ref().and_then(|n| n.author.clone()),
                news_item.as_ref().and_then(|n| n.digest.clone()),
                news_item.as_ref().and_then(|n| n.content.clone()),
                news_item.as_ref().and_then(|n| n.content_source_url.clone()),
                news_item.as_ref().map(|n| n.thumb_media_id.clone()),
                news_item.as_ref().and_then(|n| n.url.clone()).or(item.url.clone()),
            )
        }
        "video" => {
            (
                item.name.clone(),
                None,
                None,
                None,
                None,
                None,
                item.url.clone(),
            )
        }
        _ => {
            // image, voice, thumb
            (item.name.clone(), None, None, None, None, None, item.url.clone())
        }
    }
}
