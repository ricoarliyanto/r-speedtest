use crate::service::prelude::*;
use crate::model::wechat::model::mwx_templates::{
    WxTemplatesModel, WxTemplatesAdd, WxTemplatesEdit, WxTemplatesSearch, WxTemplatesDel,
};
use crate::service::wechat::wechat_sdk::wechatclient::WeChatClient;
use crate::model::wechat::args::awx_template_logs::WxTemplateLogsSearch;
use validator::Validate;

pub async fn list_tree(
    VQuery(arg): VQuery<PageParams>,
    VQuery(search): VQuery<WxTemplatesSearch>,
) -> impl IntoResponse {
    let rlist = WxTemplatesModel::list(arg, search).await;
    ApiResponse::from_result(rlist)
}

pub async fn edit(VJson(arg): VJson<WxTemplatesEdit>) -> impl IntoResponse {
    let r = WxTemplatesModel::edit(arg).await;
    ApiResponse::from_result(r)
}

pub async fn add(VJson(arg): VJson<WxTemplatesAdd>) -> impl IntoResponse {
    let r = WxTemplatesModel::add(arg).await;
    ApiResponse::from_result(r)
}

pub async fn delete(VQuery(arg): VQuery<WxTemplatesDel>) -> impl IntoResponse {
    let r = WxTemplatesModel::del(arg).await;
    ApiResponse::<String>::from_result(r)
}

/// 从微信服务器同步模板列表
pub async fn sync_templates(VJson(arg): VJson<SyncTemplatesReq>) -> impl IntoResponse {
    let account = crate::model::wechat::model::mwx_accounts::WxAccountsModel::find_by_id(arg.account_id).await;
    let account = match account {
        Ok(Some(a)) => a,
        Ok(None) => return ApiResponse::<String>::from_result(Err("公众号不存在".into())),
        Err(e) => return ApiResponse::<String>::from_result(Err(e)),
    };

    let client = WeChatClient::new(account.app_id.clone(), account.app_secret.clone());
    let result = client.get_template_list().await;
    match result {
        Ok(list) => ApiResponse::from_result(Ok(list)),
        Err(e) => ApiResponse::<String>::from_result(Err(format!("同步模板失败: {}", e).into())),
    }
}

/// 发送模板消息
pub async fn send_template(VJson(arg): VJson<SendTemplateReq>) -> impl IntoResponse {
    let account = crate::model::wechat::model::mwx_accounts::WxAccountsModel::find_by_id(arg.account_id).await;
    let account = match account {
        Ok(Some(a)) => a,
        Ok(None) => return ApiResponse::<String>::from_result(Err("公众号不存在".into())),
        Err(e) => return ApiResponse::<String>::from_result(Err(e)),
    };

    let client = WeChatClient::new(account.app_id.clone(), account.app_secret.clone());

    let result = client.send_template_message(
        &arg.openid,
        &arg.template_id,
        &arg.template_data,
        arg.url.as_deref(),
    ).await;

    match result {
        Ok(v) => ApiResponse::from_result(Ok(v)),
        Err(e) => ApiResponse::<String>::from_result(Err(format!("发送模板消息失败: {}", e).into())),
    }
}

/// 查看发送日志
pub async fn list_logs(
    VQuery(arg): VQuery<PageParams>,
    VQuery(search): VQuery<WxTemplateLogsSearch>,
) -> impl IntoResponse {
    let rlist = crate::model::wechat::model::mwx_template_logs::WxTemplateLogsModel::list(arg, search).await;
    ApiResponse::from_result(rlist)
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SyncTemplatesReq {
    pub account_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SendTemplateReq {
    pub account_id: i64,
    pub openid: String,
    pub template_id: String,
    pub template_data: serde_json::Value,
    pub url: Option<String>,
}
