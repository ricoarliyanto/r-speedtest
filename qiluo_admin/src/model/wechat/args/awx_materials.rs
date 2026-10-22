use sea_orm::prelude::DateTimeUtc;

use crate::model::prelude::*;

/// 素材列表响应
#[derive(Debug, Clone, Serialize, Deserialize, FromQueryResult, Validate)]
pub struct WxMaterialsResp {
    #[serde(with = "i64_to_string")]
    pub id: i64,
    #[serde(with = "i64_to_string")]
    pub account_id: i64,
    pub media_type: String,
    pub media_id: Option<String>,
    pub name: Option<String>,
    pub url: Option<String>,
    pub local_path: Option<String>,
    #[serde(with = "option_string_or_i64", default)]
    pub file_size: Option<i64>,
    pub content_type: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub duration: Option<i32>,
    pub description: Option<String>,
    pub title: Option<String>,
    pub introduction: Option<String>,
    pub thumb_media_id: Option<String>,
    pub thumb_url: Option<String>,
    pub content_source_url: Option<String>,
    pub digest: Option<String>,
    pub author: Option<String>,
    pub content: Option<String>,
    pub news_items: Option<String>,
    pub is_permanent: i8,
    pub sync_status: i8,
    pub synced_at: Option<DateTimeUtc>,
    pub created_at: Option<DateTimeUtc>,
    pub updated_at: Option<DateTimeUtc>,
}

/// 添加素材
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct WxMaterialsAdd {
    #[serde(with = "i64_to_string")]
    pub account_id: i64,
    pub media_type: String,
    pub media_id: Option<String>,
    pub name: Option<String>,
    pub url: Option<String>,
    pub local_path: Option<String>,
    pub file_size: Option<i64>,
    pub content_type: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub duration: Option<i32>,
    pub description: Option<String>,
    pub title: Option<String>,
    pub introduction: Option<String>,
    pub thumb_media_id: Option<String>,
    pub thumb_url: Option<String>,
    pub content_source_url: Option<String>,
    pub digest: Option<String>,
    pub author: Option<String>,
    pub content: Option<String>,
    pub news_items: Option<String>,
    pub is_permanent: Option<i8>,
    pub sync_status: Option<i8>,
    pub synced_at: Option<DateTimeUtc>,
    pub created_at: Option<DateTimeUtc>,
    pub updated_at: Option<DateTimeUtc>,
}

/// 编辑素材
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct WxMaterialsEdit {
    #[serde(with = "i64_to_string")]
    pub id: i64,
    #[serde(with = "i64_to_string")]
    pub account_id: i64,
    pub media_type: Option<String>,
    pub media_id: Option<String>,
    pub name: Option<String>,
    pub url: Option<String>,
    pub local_path: Option<String>,
    pub file_size: Option<i64>,
    pub content_type: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub duration: Option<i32>,
    pub description: Option<String>,
    pub title: Option<String>,
    pub introduction: Option<String>,
    pub thumb_media_id: Option<String>,
    pub thumb_url: Option<String>,
    pub content_source_url: Option<String>,
    pub digest: Option<String>,
    pub author: Option<String>,
    pub content: Option<String>,
    pub news_items: Option<String>,
    pub is_permanent: Option<i8>,
    pub sync_status: Option<i8>,
    pub synced_at: Option<DateTimeUtc>,
    pub updated_at: Option<DateTimeUtc>,
}

/// 删除素材
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct WxMaterialsDel {
    #[serde(with = "i64_to_string")]
    pub id: i64,
}

/// 素材搜索
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct WxMaterialsSearch {
    #[serde(with = "option_string_or_i64", default)]
    pub account_id: Option<i64>,
    pub media_type: Option<String>,
    pub name: Option<String>,
    pub is_permanent: Option<i8>,
    pub sync_status: Option<i8>,
}

/// 上传临时素材请求
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UploadTempMediaReq {
    #[serde(with = "i64_to_string")]
    pub account_id: i64,
    /// 素材类型：image/voice/video/thumb
    pub media_type: String,
}

/// 上传永久素材请求
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UploadPermanentMediaReq {
    #[serde(with = "i64_to_string")]
    pub account_id: i64,
    /// 素材类型：image/voice/video/thumb
    pub media_type: String,
    /// 视频标题（video时必填）
    pub title: Option<String>,
    /// 视频描述（video时必填）
    pub introduction: Option<String>,
}

/// 上传永久图文素材请求
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UploadNewsReq {
    #[serde(with = "i64_to_string")]
    pub account_id: i64,
    /// 图文消息条目列表
    pub articles: Vec<NewsArticleItem>,
}

/// 图文消息条目
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct NewsArticleItem {
    /// 图文消息缩略图的media_id
    pub thumb_media_id: String,
    /// 作者
    pub author: Option<String>,
    /// 标题
    pub title: String,
    /// 图文消息内容（HTML）
    pub content: String,
    /// 原文链接
    pub content_source_url: Option<String>,
    /// 摘要
    pub digest: Option<String>,
    /// 是否显示封面：0-false，1-true
    pub show_cover_pic: Option<i8>,
    /// 是否打开评论：0-不打开，1-打开
    pub need_open_comment: Option<i8>,
    /// 是否粉丝才可评论：0-所有人可评论，1-粉丝才可评论
    pub only_fans_can_comment: Option<i8>,
}

/// 从微信同步素材请求
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SyncMaterialsReq {
    #[serde(with = "i64_to_string")]
    pub account_id: i64,
    /// 素材类型：image/voice/video/news
    pub media_type: String,
    /// 从该偏移量开始同步
    #[serde(default)]
    pub offset: Option<i64>,
    /// 同步数量（最大20）
    #[serde(default)]
    pub count: Option<i64>,
}

/// 获取素材计数请求
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct MaterialCountReq {
    #[serde(with = "i64_to_string")]
    pub account_id: i64,
}

/// 删除微信素材请求
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct DeleteRemoteMediaReq {
    #[serde(with = "i64_to_string")]
    pub account_id: i64,
    /// 微信素材media_id
    pub media_id: String,
}
