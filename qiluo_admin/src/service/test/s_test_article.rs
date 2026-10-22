use crate::model::test::model::mtest_article::{
    TestArticleAdd, TestArticleDel, TestArticleEdit, TestArticleModel, TestArticleSearch,
};
use crate::service::prelude::*;

pub async fn list(
    VQuery(arg): VQuery<PageParams>,
    VQuery(search): VQuery<TestArticleSearch>,
) -> impl IntoResponse {
    let rlist = TestArticleModel::list(arg, search).await;
    ApiResponse::from_result(rlist)
}

pub async fn edit(VJson(arg): VJson<TestArticleEdit>) -> impl IntoResponse {
    let r = TestArticleModel::edit(arg).await;
    ApiResponse::from_result(r)
}

pub async fn add(VJson(arg): VJson<TestArticleAdd>) -> impl IntoResponse {
    let r = TestArticleModel::add(arg).await;
    ApiResponse::from_result(r)
}

pub async fn delete(VQuery(arg): VQuery<TestArticleDel>) -> impl IntoResponse {
    let r = TestArticleModel::del(arg).await;
    ApiResponse::from_result(r)
}