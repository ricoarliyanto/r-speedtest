//! `SeaORM` Entity for sys_mail_log

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "sys_mail_log")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, unique)]
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

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
