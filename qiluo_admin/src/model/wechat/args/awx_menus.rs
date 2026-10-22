use sea_orm::prelude::DateTimeUtc;

use crate::model::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize, FromQueryResult, Validate)]
pub struct WxMenusResp {
    #[serde(with = "i64_to_string")]
    pub id: i64,
    #[serde(with = "i64_to_string")]
    pub account_id: i64,
    #[serde(with = "option_string_or_i64", default)]
    pub parent_id: Option<i64>,
    pub menu_name: String,
    pub menu_type: String,
    pub menu_key: Option<String>,
    pub url: Option<String>,
    pub media_id: Option<String>,
    pub appid: Option<String>,
    pub pagepath: Option<String>,
    pub article_id: Option<String>,
    pub sort_order: Option<i32>,
    pub status: Option<i8>,
    pub created_at: Option<DateTimeUtc>,
    pub updated_at: Option<DateTimeUtc>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct WxMenusAdd {
    #[serde(with = "i64_to_string")]
    pub account_id: i64,
    #[serde(with = "option_string_or_i64", default)]
    pub parent_id: Option<i64>,
    pub menu_name: String,
    pub menu_type: String,
    pub menu_key: Option<String>,
    pub url: Option<String>,
    pub media_id: Option<String>,
    pub appid: Option<String>,
    pub pagepath: Option<String>,
    pub article_id: Option<String>,
    pub sort_order: Option<i32>,
    pub status: Option<i8>,
    pub created_at: Option<DateTimeUtc>,
    pub updated_at: Option<DateTimeUtc>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct WxMenusEdit {
    #[serde(with = "i64_to_string")]
    pub id: i64,
    #[serde(with = "i64_to_string")]
    pub account_id: i64,
    #[serde(with = "option_string_or_i64", default)]
    pub parent_id: Option<i64>,
    pub menu_name: String,
    pub menu_type: String,
    pub menu_key: Option<String>,
    pub url: Option<String>,
    pub media_id: Option<String>,
    pub appid: Option<String>,
    pub pagepath: Option<String>,
    pub article_id: Option<String>,
    pub sort_order: Option<i32>,
    pub status: Option<i8>,
    pub created_at: Option<DateTimeUtc>,
    pub updated_at: Option<DateTimeUtc>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct WxMenusDel {
    #[serde(with = "i64_to_string")]
    pub id: i64,
}
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct WxMenusSearch {
    #[serde(with = "option_string_or_i64", default)]
    pub account_id: Option<i64>,
    pub menu_name: Option<String>,
    pub menu_type: Option<String>,
    pub status: Option<i8>,
}

/// 菜单操作参数（同步/发布/删除远程菜单）
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct WxMenuAction {
    #[serde(with = "i64_to_string")]
    pub account_id: i64,
}
