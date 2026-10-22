use crate::model::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, FromQueryResult)]
pub struct MailTemplateRes {
    #[serde(with = "i64_to_string")]
    pub id: i64,
    pub name: String,
    pub code: String,
    pub subject: String,
    pub text_content: Option<String>,
    pub html_content: Option<String>,
    pub description: Option<String>,
    pub status: i32,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Validate)]
pub struct MailTemplateSearch {
    pub name: Option<String>,
    pub code: Option<String>,
    pub subject: Option<String>,
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct AddMailTemplateReq {
    #[validate(length(min = 1, max = 128, message = "模板名称长度1-128"))]
    pub name: String,
    #[validate(length(min = 1, max = 64, message = "模板编码长度1-64"))]
    pub code: String,
    #[validate(length(min = 1, max = 256, message = "主题长度1-256"))]
    pub subject: String,
    pub text_content: Option<String>,
    pub html_content: Option<String>,
    pub description: Option<String>,
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct EditMailTemplateReq {
    #[serde(with = "i64_to_string")]
    pub id: i64,
    #[validate(length(min = 1, max = 128, message = "模板名称长度1-128"))]
    pub name: Option<String>,
    #[validate(length(min = 1, max = 64, message = "模板编码长度1-64"))]
    pub code: Option<String>,
    #[validate(length(min = 1, max = 256, message = "主题长度1-256"))]
    pub subject: Option<String>,
    pub text_content: Option<String>,
    pub html_content: Option<String>,
    pub description: Option<String>,
    pub status: Option<i32>,
}
