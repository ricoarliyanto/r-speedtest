use sea_orm::prelude::DateTimeUtc;

use crate::model::prelude::*;

/// 手动回复消息参数
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct WxMessageReply {
    #[serde(with = "i64_to_string")]
    pub account_id: i64,
    pub openid: String,
    /// 消息类型：text / image / link
    pub msg_type: String,
    /// 文本内容（msg_type=text 时必填）
    pub content: Option<String>,
    /// 图片素材ID（msg_type=image 时必填）
    pub media_id: Option<String>,
    /// 链接标题（msg_type=link 时必填）
    pub title: Option<String>,
    /// 链接描述（msg_type=link 时可选）
    pub description: Option<String>,
    /// 链接URL（msg_type=link 时必填）
    pub url: Option<String>,
    /// 链接缩略图URL（msg_type=link 时可选）
    pub thumb_url: Option<String>,
}

/// SSE 消息流查询参数
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct WxMessageStreamQuery {
    #[serde(with = "i64_to_string")]
    pub account_id: i64,
    #[serde(default, with = "option_string_or_i64")]
    pub last_id: Option<i64>,
}

/// 会话查询参数
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct WxConversationQuery {
    #[serde(with = "i64_to_string")]
    pub account_id: i64,
    pub openid: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, FromQueryResult, Validate)]
pub struct WxMessagesResp {
    #[serde(with = "i64_to_string")]
    pub id: i64,
    #[serde(with = "i64_to_string")]
    pub account_id: i64,
    pub openid: String,
    #[serde(with = "option_string_or_i64", default)]
    pub msg_id: Option<i64>,
    pub msg_type: String,
    pub direction: i8,
    pub content: Option<String>,
    pub media_id: Option<String>,
    pub pic_url: Option<String>,
    pub voice_format: Option<String>,
    pub recognition: Option<String>,
    pub thumb_media_id: Option<String>,
    pub msg_title: Option<String>,
    pub msg_description: Option<String>,
    pub link_url: Option<String>,
    pub event_type: Option<String>,
    pub event_key: Option<String>,
    #[serde(with = "option_string_or_i64", default)]
    pub reply_msg_id: Option<i64>,
    pub is_auto_reply: Option<i8>,
    pub created_at: Option<DateTimeUtc>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Validate, Default)]
pub struct WxMessagesAdd {
    #[serde(with = "i64_to_string")]
    pub account_id: i64,
    pub openid: String,
    #[serde(with = "option_string_or_i64", default)]
    pub msg_id: Option<i64>,
    pub msg_type: String,
    pub direction: i8,
    pub content: Option<String>,
    pub media_id: Option<String>,
    pub pic_url: Option<String>,
    pub voice_format: Option<String>,
    pub recognition: Option<String>,
    pub thumb_media_id: Option<String>,
    pub msg_title: Option<String>,
    pub msg_description: Option<String>,
    pub link_url: Option<String>,
    pub event_type: Option<String>,
    pub event_key: Option<String>,
    #[serde(with = "option_string_or_i64", default)]
    pub reply_msg_id: Option<i64>,
    pub is_auto_reply: Option<i8>,
    pub created_at: Option<DateTimeUtc>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct WxMessagesEdit {
    #[serde(with = "i64_to_string")]
    pub id: i64,
    #[serde(with = "i64_to_string")]
    pub account_id: i64,
    pub openid: String,
    #[serde(with = "option_string_or_i64", default)]
    pub msg_id: Option<i64>,
    pub msg_type: String,
    pub direction: i8,
    pub content: Option<String>,
    pub media_id: Option<String>,
    pub pic_url: Option<String>,
    pub voice_format: Option<String>,
    pub recognition: Option<String>,
    pub thumb_media_id: Option<String>,
    pub msg_title: Option<String>,
    pub msg_description: Option<String>,
    pub link_url: Option<String>,
    pub event_type: Option<String>,
    pub event_key: Option<String>,
    #[serde(with = "option_string_or_i64", default)]
    pub reply_msg_id: Option<i64>,
    pub is_auto_reply: Option<i8>,
    pub created_at: Option<DateTimeUtc>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct WxMessagesDel {
    #[serde(with = "i64_to_string")]
    pub id: i64,
}
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct WxMessagesSearch {
    #[serde(with = "option_string_or_i64", default)]
    pub account_id: Option<i64>,
    pub openid: Option<String>,
    pub msg_type: Option<String>,
    pub direction: Option<i8>,
}
