//! `SeaORM` Entity for test_article — 手动维护，位于 extend/ 不受 sea-orm-codegen 覆盖

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "test_article")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    /// 外键 → BelongsTo TestCategory (Select/远程搜索)
    pub category_id: i64,
    /// 标题 → Input
    pub title: String,
    /// 内容 → Textarea
    pub content: String,
    /// 作者 → Input
    pub author: Option<String>,
    /// 密码 → InputPassword
    pub password: String,
    /// 发布状态 → Switch
    pub is_published: bool,
    /// 浏览量 → Input (i64, JS 精度保护)
    pub view_count: i64,
    /// 下载次数 → InputNumber
    pub download_count: u32,
    /// 评分 → InputNumber
    pub rating: f64,
    /// 封面 → Upload
    pub cover: Option<String>,
    /// 发布时间 → DatePicker
    pub published_at: Option<DateTime>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::test_category::Entity",
        from = "Column::CategoryId",
        to = "super::test_category::Column::Id"
    )]
    TestCategory,
}

impl Related<super::test_category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TestCategory.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
