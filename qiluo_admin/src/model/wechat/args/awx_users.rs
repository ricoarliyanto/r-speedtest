use sea_orm::prelude::DateTimeUtc;

use crate::model::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize, FromQueryResult, Validate)]
pub struct WxUsersResp {
    #[serde(with = "i64_to_string")]
    pub id: i64,
    #[serde(with = "i64_to_string")]
    pub account_id: i64,
    pub openid: String,
    pub unionid: Option<String>,
    pub nickname: Option<String>,
    pub sex: Option<i8>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub province: Option<String>,
    pub language: Option<String>,
    pub headimgurl: Option<String>,
    pub subscribe_time: Option<DateTimeUtc>,
    pub unsubscribe_time: Option<DateTimeUtc>,
    pub subscribe_status: Option<i8>,
    pub remark: Option<String>,
    pub subscribe_scene: Option<String>,
    pub qr_scene: Option<String>,
    pub qr_scene_str: Option<String>,
    pub last_interact_time: Option<DateTimeUtc>,
    pub message_count: Option<i32>,
    pub created_at: Option<DateTimeUtc>,
    pub updated_at: Option<DateTimeUtc>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Validate, Default)]
pub struct WxUsersAdd {
    #[serde(with = "i64_to_string")]
    pub account_id: i64,
    pub openid: String,
    pub unionid: Option<String>,
    pub nickname: Option<String>,
    pub sex: Option<i8>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub province: Option<String>,
    pub language: Option<String>,
    pub headimgurl: Option<String>,
    pub subscribe_time: Option<DateTimeUtc>,
    pub unsubscribe_time: Option<DateTimeUtc>,
    pub subscribe_status: Option<i8>,
    pub remark: Option<String>,
    pub subscribe_scene: Option<String>,
    pub qr_scene: Option<String>,
    pub qr_scene_str: Option<String>,
    pub last_interact_time: Option<DateTimeUtc>,
    pub message_count: Option<i32>,
    pub created_at: Option<DateTimeUtc>,
    pub updated_at: Option<DateTimeUtc>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct WxUsersEdit {
    #[serde(with = "i64_to_string")]
    pub id: i64,
    #[serde(with = "i64_to_string")]
    pub account_id: i64,
    pub openid: String,
    pub unionid: Option<String>,
    pub nickname: Option<String>,
    pub sex: Option<i8>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub province: Option<String>,
    pub language: Option<String>,
    pub headimgurl: Option<String>,
    pub subscribe_time: Option<DateTimeUtc>,
    pub unsubscribe_time: Option<DateTimeUtc>,
    pub subscribe_status: Option<i8>,
    pub remark: Option<String>,
    pub subscribe_scene: Option<String>,
    pub qr_scene: Option<String>,
    pub qr_scene_str: Option<String>,
    pub last_interact_time: Option<DateTimeUtc>,
    pub message_count: Option<i32>,
    pub created_at: Option<DateTimeUtc>,
    pub updated_at: Option<DateTimeUtc>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct WxUsersDel {
    #[serde(with = "i64_to_string")]
    pub id: i64,
}
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct WxUsersSearch {
    #[serde(with = "option_string_or_i64", default)]
    pub account_id: Option<i64>,
    pub openid: Option<String>,
    pub nickname: Option<String>,
    pub sex: Option<i8>,
    pub subscribe_status: Option<i8>,
}
