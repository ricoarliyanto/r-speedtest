use crate::model::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, FromQueryResult, Validate)]
pub struct SysPostResp {
    #[serde(with = "i64_to_string")]
    pub post_id: i64,
    #[serde(with = "i64_to_string")]
    pub dept_id: i64,
    pub post_code: String,
    pub post_category: Option<String>,
    pub post_name: String,
    pub post_sort: i32,
    pub status: String,
    pub remark: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SysPostSearch {
    pub post_name: Option<String>,
    pub post_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SysPostAdd {
    pub dept_id: i64,
    pub post_code: String,
    pub post_category: Option<String>,
    pub post_name: String,
    pub post_sort: i32,
    pub status: String,
    pub remark: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SysPostEdit {
    #[serde(with = "i64_to_string")]
    pub post_id: i64,
    pub dept_id: i64,
    pub post_code: String,
    pub post_category: Option<String>,
    pub post_name: String,
    pub post_sort: i32,
    pub status: String,
    pub remark: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SysPostDel {
    #[serde(with = "i64_to_string")]
    pub post_id: i64,
}
