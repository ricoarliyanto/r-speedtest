use crate::service::prelude::*;
use crate::model::wechat::model::mwx_accounts::{
    WxAccountsModel, WxAccountsAdd, WxAccountsEdit, WxAccountsSearch, WxAccountsDel,
};
pub async fn list_tree(
    VQuery(arg): VQuery<PageParams>,
    VQuery(search): VQuery<WxAccountsSearch>,
) -> impl IntoResponse {
    let rlist = WxAccountsModel::list(arg, search).await;
    ApiResponse::from_result(rlist)
}
pub async fn edit(VJson(arg): VJson<WxAccountsEdit>) -> impl IntoResponse {
    let r = WxAccountsModel::edit(arg).await;
    ApiResponse::from_result(r)
}
pub async fn add(VJson(arg): VJson<WxAccountsAdd>) -> impl IntoResponse {
    let r = WxAccountsModel::add(arg).await;
    ApiResponse::from_result(r)
}
pub async fn delete(VQuery(arg): VQuery<WxAccountsDel>) -> impl IntoResponse {
    let r = WxAccountsModel::del(arg).await;
    ApiResponse::<String>::from_result(r)
}
