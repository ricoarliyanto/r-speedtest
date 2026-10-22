use crate::model::test::model::mtest_category::{
    TestCategoryAdd, TestCategoryDel, TestCategoryEdit, TestCategoryModel, TestCategorySearch,
};
use crate::service::prelude::*;

pub async fn list(
    VQuery(arg): VQuery<PageParams>,
    VQuery(search): VQuery<TestCategorySearch>,
) -> impl IntoResponse {
    let rlist = TestCategoryModel::list(arg, search).await;
    ApiResponse::from_result(rlist)
}

pub async fn edit(VJson(arg): VJson<TestCategoryEdit>) -> impl IntoResponse {
    let r = TestCategoryModel::edit(arg).await;
    ApiResponse::from_result(r)
}

pub async fn add(VJson(arg): VJson<TestCategoryAdd>) -> impl IntoResponse {
    let r = TestCategoryModel::add(arg).await;
    ApiResponse::from_result(r)
}

pub async fn delete(VQuery(arg): VQuery<TestCategoryDel>) -> impl IntoResponse {
    let r = TestCategoryModel::del(arg).await;
    ApiResponse::from_result(r)
}