//! `SeaORM` Entity for test_category — 手动维护，位于 extend/ 不受 sea-orm-codegen 覆盖

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "test_category")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    /// 分类名称 → Input
    pub name: String,
    /// 分类描述 → Textarea
    pub description: Option<String>,
    /// 排序 → InputNumber
    pub sort: i32,
    /// 状态 → Select
    pub status: String,
    /// 封面图 → Upload
    pub cover: Option<String>,
    /// 是否启用 → Switch
    pub is_active: bool,
    /// 权重 → InputNumber
    pub weight: i32,
    /// 浏览量 → Input (i64, JS 精度保护)
    pub view_count: i64,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::test_article::Entity")]
    TestArticle,
}

impl Related<super::test_article::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TestArticle.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
