use crate::model::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, FromQueryResult, Validate)]
pub struct TestCategoryResp {
    #[serde(with = "i64_to_string")]
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub sort: i32,
    pub status: String,
    pub cover: Option<String>,
    pub is_active: bool,
    pub weight: i32,
    #[serde(with = "i64_to_string")]
    pub view_count: i64,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct TestCategorySearch {
    pub name: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub cover: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct TestCategoryAdd {
    pub name: String,
    pub description: Option<String>,
    pub sort: i32,
    pub status: String,
    pub cover: Option<String>,
    pub is_active: bool,
    pub weight: i32,
    #[serde(with = "i64_to_string")]
    pub view_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct TestCategoryEdit {
    #[serde(with = "i64_to_string")]
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub sort: i32,
    pub status: String,
    pub cover: Option<String>,
    pub is_active: bool,
    pub weight: i32,
    #[serde(with = "i64_to_string")]
    pub view_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct TestCategoryDel {
    #[serde(with = "i64_to_string")]
    pub id: i64,
}