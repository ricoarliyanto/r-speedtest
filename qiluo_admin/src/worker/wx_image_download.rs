use crate::common::error::Result;
use crate::model::wechat::entity::wx_messages;
use crate::service::prelude::*;
use crate::worker::common::{Worker, WorkerOpts};
use crate::worker::AppWorker;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sea_orm::Set;

/// Worker 参数：消息 ID + 原始图片 URL
#[derive(Deserialize, Serialize, Clone, Default)]
pub struct WxImageDownloadMsg {
    pub msg_id: i64,
    pub pic_url: String,
}

#[derive(Clone)]
pub struct WxImageDownloadWorker {}

impl AppWorker<WxImageDownloadMsg> for WxImageDownloadWorker {
    fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Worker<WxImageDownloadMsg> for WxImageDownloadWorker {
    fn opts() -> WorkerOpts<WxImageDownloadMsg, Self> {
        WorkerOpts::new().queue("default")
    }

    async fn perform(&self, arg: WxImageDownloadMsg) -> Result<()> {
        tracing::info!("开始下载微信图片: msg_id={}, url={}", arg.msg_id, arg.pic_url);

        // 下载图片
        let client = reqwest::Client::new();
        let resp = match client.get(&arg.pic_url).send().await {
            Ok(r) => r,
            Err(e) => {
                tracing::error!("下载微信图片失败: {:?}", e);
                return Err(e.into());
            }
        };

        // 先从 headers 提取 content_type（借用），再 move resp 读 bytes
        let content_type = resp
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("image/jpeg")
            .to_string(); // clone 为 String，释放 borrow

        let bytes = match resp.bytes().await {
            Ok(b) => b,
            Err(e) => {
                tracing::error!("读取微信图片数据失败: {:?}", e);
                return Err(e.into());
            }
        };

        // 保存到本地
        let server_config = APPCOFIG.server.clone();
        let now = chrono::Local::now();
        let ext = match content_type.as_str() {
            "image/png" => ".png",
            "image/gif" => ".gif",
            _ => ".jpg",
        };

        let rel_dir = format!("wx_images/{}", now.format("%Y-%m"));
        let file_path_t = format!("{}/{}", server_config.static_dir, rel_dir);
        tokio::fs::create_dir_all(&file_path_t).await?;

        let fid = GID().await;
        let file_name = format!("{}-{}{}", now.format("%d"), fid, ext);
        let file_path = format!("{}/{}", file_path_t, file_name);

        let mut file = tokio::fs::File::create(&file_path).await?;
        use tokio::io::AsyncWriteExt;
        file.write_all(&bytes).await?;

        // 生成可访问的 URL
        let static_dir = server_config
            .static_dir
            .strip_prefix("data/")
            .unwrap_or(&server_config.static_dir);
        let local_url = format!(
            "{}/{}/{}/{}",
            server_config.domainname, static_dir, rel_dir, file_name
        );

        tracing::info!("微信图片下载完成，本地地址: {}", local_url);

        // 更新数据库 pic_url
        let db = DB().await;
        let msg = wx_messages::Entity::find_by_id(arg.msg_id).one(db).await?;
        if let Some(model) = msg {
            let mut active_model: wx_messages::ActiveModel = model.into();
            active_model.pic_url = Set(Some(local_url.clone()));
            active_model.update(db).await?;
            tracing::info!("已更新消息 pic_url: msg_id={}", arg.msg_id);
        } else {
            tracing::warn!("未找到消息记录: msg_id={}", arg.msg_id);
        }

        Ok(())
    }
}