use crate::model::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, FromQueryResult, Validate)]
pub struct TestArticleResp {
    #[serde(with = "i64_to_string")]
    pub id: i64,
    #[serde(with = "i64_to_string")]
    pub category_id: i64,
    pub title: String,
    pub content: String,
    pub author: Option<String>,
    pub password: String,
    pub is_published: bool,
    #[serde(with = "i64_to_string")]
    pub view_count: i64,
    pub download_count: u32,
    pub rating: f64,
    pub cover: Option<String>,
    pub published_at: Option<DateTime>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct TestArticleSearch {
    pub title: Option<String>,
    pub content: Option<String>,
    pub author: Option<String>,
    pub password: Option<String>,
    pub cover: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct TestArticleAdd {
    #[serde(with = "i64_to_string")]
    pub category_id: i64,
    pub title: String,
    pub content: String,
    pub author: Option<String>,
    pub password: String,
    pub is_published: bool,
    #[serde(with = "i64_to_string")]
    pub view_count: i64,
    pub download_count: u32,
    pub rating: f64,
    pub cover: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct TestArticleEdit {
    #[serde(with = "i64_to_string")]
    pub id: i64,
    #[serde(with = "i64_to_string")]
    pub category_id: i64,
    pub title: String,
    pub content: String,
    pub author: Option<String>,
    pub password: String,
    pub is_published: bool,
    #[serde(with = "i64_to_string")]
    pub view_count: i64,
    pub download_count: u32,
    pub rating: f64,
    pub cover: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct TestArticleDel {
    #[serde(with = "i64_to_string")]
    pub id: i64,
}