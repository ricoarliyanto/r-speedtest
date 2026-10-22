use sea_orm::prelude::DateTimeUtc;

use crate::model::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize, FromQueryResult, Validate)]
pub struct WxTemplateLogsResp {
    #[serde(with = "i64_to_string")]
    pub id: i64,
    #[serde(with = "i64_to_string")]
    pub account_id: i64,
    pub template_id: String,
    pub openid: String,
    pub template_data: Option<String>,
    pub url: Option<String>,
    pub miniprogram_appid: Option<String>,
    pub miniprogram_pagepath: Option<String>,
    pub msg_id: Option<i64>,
    pub errcode: Option<i32>,
    pub errmsg: Option<String>,
    pub status: Option<i8>,
    pub created_at: Option<DateTimeUtc>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct WxTemplateLogsSearch {
    pub account_id: Option<i64>,
    pub template_id: Option<String>,
    pub openid: Option<String>,
    pub status: Option<i8>,
}
