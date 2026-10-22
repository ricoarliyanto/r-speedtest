use crate::service::prelude::*;
use crate::model::wechat::model::mwx_users::{
    WxUsersModel, WxUsersAdd, WxUsersEdit, WxUsersSearch, WxUsersDel,
};
pub async fn list_tree(
    VQuery(arg): VQuery<PageParams>,
    VQuery(search): VQuery<WxUsersSearch>,
) -> impl IntoResponse {
    let rlist = WxUsersModel::list(arg, search).await;
    ApiResponse::from_result(rlist)
}
pub async fn edit(VJson(arg): VJson<WxUsersEdit>) -> impl IntoResponse {
    let r = WxUsersModel::edit(arg).await;
    ApiResponse::from_result(r)
}
pub async fn add(VJson(arg): VJson<WxUsersAdd>) -> impl IntoResponse {
    let r = WxUsersModel::add(arg).await;
    ApiResponse::from_result(r)
}
pub async fn delete(VQuery(arg): VQuery<WxUsersDel>) -> impl IntoResponse {
    let r = WxUsersModel::del(arg).await;
    ApiResponse::from_result(r)
}
