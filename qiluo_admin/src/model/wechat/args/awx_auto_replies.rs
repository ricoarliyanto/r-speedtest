use sea_orm::prelude::DateTimeUtc;

use crate::model::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize, FromQueryResult, Validate)]
pub struct WxAutoRepliesResp {
    #[serde(with = "i64_to_string")]
    pub id: i64,
    #[serde(with = "i64_to_string")]
    pub account_id: i64,
    pub reply_type: i8,
    pub keyword: Option<String>,
    pub match_type: Option<i8>,
    pub message_type: String,
    pub content: Option<String>,
    pub media_id: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub pic_url: Option<String>,
    pub url: Option<String>,
    pub music_url: Option<String>,
    pub hq_music_url: Option<String>,
    pub thumb_media_id: Option<String>,
    pub status: Option<i8>,
    pub priority: Option<i32>,
    pub created_at: Option<DateTimeUtc>,
    pub updated_at: Option<DateTimeUtc>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct WxAutoRepliesAdd {
    #[serde(with = "i64_to_string")]
    pub account_id: i64,
    pub reply_type: i8,
    pub keyword: Option<String>,
    pub match_type: Option<i8>,
    pub message_type: String,
    pub content: Option<String>,
    pub media_id: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub pic_url: Option<String>,
    pub url: Option<String>,
    pub music_url: Option<String>,
    pub hq_music_url: Option<String>,
    pub thumb_media_id: Option<String>,
    pub status: Option<i8>,
    pub priority: Option<i32>,
    pub created_at: Option<DateTimeUtc>,
    pub updated_at: Option<DateTimeUtc>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct WxAutoRepliesEdit {
    #[serde(with = "i64_to_string")]
    pub id: i64,
    #[serde(with = "i64_to_string")]
    pub account_id: i64,
    pub reply_type: i8,
    pub keyword: Option<String>,
    pub match_type: Option<i8>,
    pub message_type: String,
    pub content: Option<String>,
    pub media_id: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub pic_url: Option<String>,
    pub url: Option<String>,
    pub music_url: Option<String>,
    pub hq_music_url: Option<String>,
    pub thumb_media_id: Option<String>,
    pub status: Option<i8>,
    pub priority: Option<i32>,
    pub created_at: Option<DateTimeUtc>,
    pub updated_at: Option<DateTimeUtc>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct WxAutoRepliesDel {
    #[serde(with = "i64_to_string")]
    pub id: i64,
}
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct WxAutoRepliesSearch {
    #[serde(with = "option_string_or_i64", default)]
    pub account_id: Option<i64>,
    pub reply_type: Option<i8>,
    pub keyword: Option<String>,
    pub message_type: Option<String>,
    pub status: Option<i8>,
}
