use super::web_path::{WebPath, WebPathType};
use crate::service::test::*;

use axum::{
    routing::{delete, get, post, put},
    Router,
};
pub fn router_test() -> WebPath {
    WebPath::new().nest(
        "/test",
        WebPath::new()
            .nest("/test_api", test_test_api())
            .nest("/test_data_scope", test_data_scope_api())
              .nest("/test_article", test_test_article())
              .nest("/test_category", test_test_category()),
            
    )
}
fn test_data_scope_api() -> WebPath {
    WebPath::new()
        .route(
            "/list",
            WebPathType::Get,
            Some("list"),
            get(s_test_data_scope::list),
        )
        .route(
            "/edit",
            WebPathType::Put,
            Some("edit"),
            put(s_test_data_scope::edit),
        )
        .route(
            "/add",
            WebPathType::Post,
            Some("add"),
            post(s_test_data_scope::add),
        )
        .route(
            "/del",
            WebPathType::Delete,
            Some("Delete"),
            delete(s_test_data_scope::delete),
        )
}

pub fn white_test() -> Router {
    Router::new()
}

fn test_test_api() -> WebPath {
    WebPath::new()
        .route(
            "/list",
            WebPathType::Get,
            Some("获取列表"),
            get(s_test_api::list),
        )
        .route(
            "/edit",
            WebPathType::Put,
            Some("编辑TestApi"),
            put(s_test_api::edit),
        )
        .route(
            "/add",
            WebPathType::Post,
            Some("添加TestApi"),
            post(s_test_api::add),
        )
        .route(
            "/del",
            WebPathType::Delete,
            Some("删除TestApi"),
            delete(s_test_api::delete),
        )
        .route(
            "/db_index_test", 
            WebPathType::Put,
            Some("db_index_test"),
            put(s_test_api::db_index_test),
        )
        .route(
            "/db_name_test",
            WebPathType::Put,
            Some("db_name_test"),
            put(s_test_api::db_name_test),
        )
        .route(
            "/db_read_write_test",
            WebPathType::Put,
            Some("db_read_write_test"),
            put(s_test_api::db_read_write_test),
        )
        .route(
            "/db_auto_test",
            WebPathType::Put,
            Some("db_auto_test"),
            put(s_test_api::db_auto_test),
        )
}

fn test_test_article() -> WebPath {
    WebPath::new()
        .route("/list", WebPathType::Get, Some("获取TestArticle列表"), get(s_test_article::list))
        .route("/edit", WebPathType::Put, Some("编辑TestArticle"), put(s_test_article::edit))
        .route("/add", WebPathType::Post, Some("添加TestArticle"), post(s_test_article::add))
        .route("/del", WebPathType::Delete, Some("删除TestArticle"), delete(s_test_article::delete))
}

fn test_test_category() -> WebPath {
    WebPath::new()
        .route("/list", WebPathType::Get, Some("获取TestCategory列表"), get(s_test_category::list))
        .route("/edit", WebPathType::Put, Some("编辑TestCategory"), put(s_test_category::edit))
        .route("/add", WebPathType::Post, Some("添加TestCategory"), post(s_test_category::add))
        .route("/del", WebPathType::Delete, Some("删除TestCategory"), delete(s_test_category::delete))
}
