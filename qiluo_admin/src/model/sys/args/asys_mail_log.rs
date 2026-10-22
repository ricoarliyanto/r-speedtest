use crate::model::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, FromQueryResult)]
pub struct MailLogRes {
    pub id: i64,
    pub recipient: String,
    pub subject: String,
    pub content_text: Option<String>,
    pub content_html: Option<String>,
    pub from_addr: Option<String>,
    pub reply_to: Option<String>,
    pub status: String,
    pub error_message: Option<String>,
    pub mail_type: Option<String>,
    pub created_by: Option<i64>,
    pub created_at: DateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Validate)]
pub struct MailLogSearch {
    pub recipient: Option<String>,
    pub subject: Option<String>,
    pub status: Option<String>,
    pub mail_type: Option<String>,
}
