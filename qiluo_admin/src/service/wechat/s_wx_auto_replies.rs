use crate::service::prelude::*;
use crate::model::wechat::model::mwx_auto_replies::{
    WxAutoRepliesModel, WxAutoRepliesAdd, WxAutoRepliesEdit, WxAutoRepliesSearch, WxAutoRepliesDel,
};
pub async fn list_tree(
    VQuery(arg): VQuery<PageParams>,
    VQuery(search): VQuery<WxAutoRepliesSearch>,
) -> impl IntoResponse {
    let rlist = WxAutoRepliesModel::list(arg, search).await;
    ApiResponse::from_result(rlist)
}
pub async fn edit(VJson(arg): VJson<WxAutoRepliesEdit>) -> impl IntoResponse {
    let r = WxAutoRepliesModel::edit(arg).await;
    ApiResponse::from_result(r)
}
pub async fn add(VJson(arg): VJson<WxAutoRepliesAdd>) -> impl IntoResponse {
    let r = WxAutoRepliesModel::add(arg).await;
    ApiResponse::from_result(r)
}
pub async fn delete(VQuery(arg): VQuery<WxAutoRepliesDel>) -> impl IntoResponse {
    let r = WxAutoRepliesModel::del(arg).await;
    ApiResponse::from_result(r)
}
