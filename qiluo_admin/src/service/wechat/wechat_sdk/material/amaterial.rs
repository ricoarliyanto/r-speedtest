use serde::{Deserialize, Serialize};

/// 微信上传临时素材响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadTempMediaResponse {
    #[serde(rename = "type")]
    pub media_type: String,
    pub media_id: String,
    pub created_at: i64,
}

/// 微信上传永久图片响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadImgResponse {
    pub url: String,
}

/// 微信上传永久素材响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadPermanentMediaResponse {
    pub media_id: String,
    pub url: Option<String>,
}

/// 微信上传图文素材响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadNewsResponse {
    pub media_id: String,
}

/// 微信获取素材列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchGetMaterialResponse {
    pub total_count: i64,
    pub item_count: i64,
    pub item: Vec<MaterialItem>,
}

/// 素材条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialItem {
    pub media_id: String,
    pub name: Option<String>,
    pub update_time: Option<i64>,
    pub url: Option<String>,
    pub content: Option<NewsContent>,
    pub news_item: Option<Vec<NewsItemDetail>>,
}

/// 图文消息内容
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsContent {
    pub news_item: Vec<NewsItemDetail>,
}

/// 图文消息条目详情
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsItemDetail {
    pub title: String,
    pub author: Option<String>,
    pub digest: Option<String>,
    pub content: Option<String>,
    pub content_source_url: Option<String>,
    pub thumb_media_id: String,
    pub thumb_url: Option<String>,
    pub url: Option<String>,
    pub show_cover_pic: Option<i64>,
    pub need_open_comment: Option<i64>,
    pub only_fans_can_comment: Option<i64>,
}

/// 微信素材计数响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialCountResponse {
    pub voice_count: i64,
    pub video_count: i64,
    pub image_count: i64,
    pub news_count: i64,
}

/// 上传永久视频素材请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadVideoDescription {
    pub title: String,
    pub introduction: String,
}

/// 修改永久图文素材请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateNewsRequest {
    pub media_id: String,
    pub index: i64,
    pub articles: NewsArticleUpdate,
}

/// 修改图文素材条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsArticleUpdate {
    pub title: Option<String>,
    pub author: Option<String>,
    pub digest: Option<String>,
    pub content: Option<String>,
    pub content_source_url: Option<String>,
    pub thumb_media_id: Option<String>,
    pub show_cover_pic: Option<i64>,
    pub need_open_comment: Option<i64>,
    pub only_fans_can_comment: Option<i64>,
}

/// 新增永久图文素材的条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsArticle {
    pub thumb_media_id: String,
    pub author: Option<String>,
    pub title: String,
    pub content: String,
    pub content_source_url: Option<String>,
    pub digest: Option<String>,
    pub show_cover_pic: Option<i64>,
    pub need_open_comment: Option<i64>,
    pub only_fans_can_comment: Option<i64>,
}
