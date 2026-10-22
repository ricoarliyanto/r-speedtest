use sea_orm::prelude::DateTimeUtc;

use crate::model::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize, FromQueryResult, Validate)]
pub struct WxTemplatesResp {
    #[serde(with = "i64_to_string")]
    pub id: i64,
    #[serde(with = "i64_to_string")]
    pub account_id: i64,
    pub template_id: String,
    pub title: Option<String>,
    pub industry: Option<String>,
    pub content: Option<String>,
    pub example: Option<String>,
    pub template_type: Option<String>,
    pub status: Option<i8>,
    pub created_at: Option<DateTimeUtc>,
    pub updated_at: Option<DateTimeUtc>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct WxTemplatesAdd {
    pub account_id: i64,
    pub template_id: String,
    pub title: Option<String>,
    pub industry: Option<String>,
    pub content: Option<String>,
    pub example: Option<String>,
    pub template_type: Option<String>,
    pub status: Option<i8>,
    pub created_at: Option<DateTimeUtc>,
    pub updated_at: Option<DateTimeUtc>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct WxTemplatesEdit {
    #[serde(with = "i64_to_string")]
    pub id: i64,
    #[serde(with = "i64_to_string")]
    pub account_id: i64,
    pub template_id: String,
    pub title: Option<String>,
    pub industry: Option<String>,
    pub content: Option<String>,
    pub example: Option<String>,
    pub template_type: Option<String>,
    pub status: Option<i8>,
    pub created_at: Option<DateTimeUtc>,
    pub updated_at: Option<DateTimeUtc>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct WxTemplatesDel {
    #[serde(with = "i64_to_string")]
    pub id: i64,
}
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct WxTemplatesSearch {
    pub account_id: Option<i64>,
    pub template_id: Option<String>,
    pub title: Option<String>,
    pub status: Option<i8>,
}
