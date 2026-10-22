use sea_orm::prelude::DateTimeUtc;

use crate::model::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize, FromQueryResult, Validate)]
pub struct WxAccountsResp {
    #[serde(with = "i64_to_string")]
    pub id: i64,
    pub app_id: String,
    pub app_secret: String,
    pub account_name: String,
    pub account_type: i8,
    pub original_id: Option<String>,
    pub wechat_id: Option<String>,
    pub status: Option<i8>,
    pub message_mode: i8,
    pub access_token: Option<String>,
    pub token_expires_at: Option<DateTimeUtc>,
    pub server_url: Option<String>,
    pub token: Option<String>,
    pub encoding_aes_key: Option<String>,
    pub created_at: Option<DateTimeUtc>,
    pub updated_at: Option<DateTimeUtc>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct WxAccountsAdd {
    pub app_id: String,
    pub app_secret: String,
    pub account_name: String,
    pub account_type: i8,
    pub original_id: Option<String>,
    pub wechat_id: Option<String>,
    pub status: Option<i8>,
    pub message_mode: i8,
    pub access_token: Option<String>,
    pub token_expires_at: Option<DateTimeUtc>,
    pub server_url: Option<String>,
    pub token: Option<String>,
    pub encoding_aes_key: Option<String>,
    pub created_at: Option<DateTimeUtc>,
    pub updated_at: Option<DateTimeUtc>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct WxAccountsEdit {
    #[serde(with = "i64_to_string")]
    pub id: i64,
    pub app_id: String,
    pub app_secret: String,
    pub account_name: String,
    pub account_type: i8,
    pub original_id: Option<String>,
    pub wechat_id: Option<String>,
    pub status: Option<i8>,
    pub message_mode: i8,
    pub access_token: Option<String>,
    pub token_expires_at: Option<DateTimeUtc>,
    pub server_url: Option<String>,
    pub token: Option<String>,
    pub encoding_aes_key: Option<String>,
    pub created_at: Option<DateTimeUtc>,
    pub updated_at: Option<DateTimeUtc>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct WxAccountsDel {
    #[serde(with = "i64_to_string")]
    pub id: i64,
}
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct WxAccountsSearch {
    pub app_id: Option<String>,
    pub account_name: Option<String>,
    pub account_type: Option<i8>,
    pub original_id: Option<String>,
    pub wechat_id: Option<String>,
    pub status: Option<i8>,
}
