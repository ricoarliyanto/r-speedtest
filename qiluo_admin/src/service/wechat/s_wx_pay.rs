use crate::service::prelude::*;
use crate::model::wechat::model::mwx_pay_orders::{WxPayOrdersModel, WxPayOrdersSearch, WxPayOrdersAddReq};
use crate::model::wechat::model::mwx_pay_refunds::{WxPayRefundsModel, WxPayRefundsSearch, WxPayRefundsAddReq};
use crate::service::wechat::wechat_sdk::pay::payclient::WechatPayClient;
use crate::service::wechat::wechat_sdk::pay::apay::TradeType;
use validator::Validate;

pub async fn list_orders(
    VQuery(arg): VQuery<PageParams>,
    VQuery(search): VQuery<WxPayOrdersSearch>,
) -> impl IntoResponse {
    let rlist = WxPayOrdersModel::list(arg, search).await;
    ApiResponse::from_result(rlist)
}

pub async fn list_refunds(
    VQuery(arg): VQuery<PageParams>,
    VQuery(search): VQuery<WxPayRefundsSearch>,
) -> impl IntoResponse {
    let rlist = WxPayRefundsModel::list(arg, search).await;
    ApiResponse::from_result(rlist)
}

/// 创建支付订单 (统一下单)
pub async fn create_order(VJson(arg): VJson<CreatePayOrderReq>) -> impl IntoResponse {
    let account = crate::model::wechat::model::mwx_accounts::WxAccountsModel::find_by_id(arg.account_id).await;
    let account = match account {
        Ok(Some(a)) => a,
        Ok(None) => return ApiResponse::<String>::from_result(Err("公众号不存在".into())),
        Err(e) => return ApiResponse::<String>::from_result(Err(e)),
    };

    let pay_client = WechatPayClient::new(
        account.app_id.clone(),
        arg.mch_id.clone(),
        arg.api_key.clone(),
    );

    let trade_type = match arg.trade_type.as_str() {
        "JSAPI" => TradeType::Jsapi,
        "NATIVE" => TradeType::Native,
        "APP" => TradeType::App,
        "MWEB" => TradeType::Mweb,
        _ => TradeType::Jsapi,
    };

    let time_expire = arg.time_expire.map(|t| {
        t.format("%Y%m%d%H%M%S").to_string()
    });

    let result = pay_client.unified_order(
        &arg.body,
        &arg.out_trade_no,
        arg.total_fee,
        arg.spbill_create_ip.as_deref().unwrap_or("127.0.0.1"),
        &arg.notify_url,
        &trade_type,
        arg.openid.as_deref(),
        arg.attach.as_deref(),
        time_expire.as_deref(),
    ).await;

    match result {
        Ok(resp) => {
            if resp.return_code == "SUCCESS" && resp.result_code.as_deref() == Some("SUCCESS") {
                let prepay_id = resp.prepay_id.clone().unwrap_or_default();
                let code_url = resp.code_url.clone();

                // 保存订单到数据库
                let order_arg = WxPayOrdersAddReq {
                    account_id: arg.account_id,
                    out_trade_no: arg.out_trade_no.clone(),
                    openid: arg.openid.clone().unwrap_or_default(),
                    body: arg.body.clone(),
                    total_fee: arg.total_fee,
                    fee_type: arg.fee_type.clone(),
                    trade_type: arg.trade_type.clone(),
                    spbill_create_ip: arg.spbill_create_ip.clone(),
                    notify_url: arg.notify_url.clone(),
                    mch_id: arg.mch_id.clone(),
                    attach: arg.attach.clone(),
                    time_expire: arg.time_expire,
                };
                let _ = WxPayOrdersModel::add(order_arg).await;

                // 如果是JSAPI支付，生成前端参数
                let jsapi_params = if trade_type.as_str() == "JSAPI" && !prepay_id.is_empty() {
                    Some(pay_client.jsapi_pay_params(&prepay_id))
                } else {
                    None
                };

                ApiResponse::from_result(Ok(serde_json::json!({
                    "prepay_id": prepay_id,
                    "code_url": code_url,
                    "jsapi_params": jsapi_params,
                })))
            } else {
                let err_code = resp.err_code.unwrap_or_default();
                let err_desc = resp.err_code_des.unwrap_or_default();
                ApiResponse::<String>::from_result(Err(format!("下单失败: {} - {}", err_code, err_desc).into()))
            }
        }
        Err(e) => ApiResponse::<String>::from_result(Err(format!("下单请求失败: {}", e).into())),
    }
}

/// 查询订单状态
pub async fn query_order(VJson(arg): VJson<QueryOrderReq>) -> impl IntoResponse {
    let account = crate::model::wechat::model::mwx_accounts::WxAccountsModel::find_by_id(arg.account_id).await;
    let account = match account {
        Ok(Some(a)) => a,
        Ok(None) => return ApiResponse::<String>::from_result(Err("公众号不存在".into())),
        Err(e) => return ApiResponse::<String>::from_result(Err(e)),
    };

    let pay_client = WechatPayClient::new(
        account.app_id.clone(),
        arg.mch_id.clone(),
        arg.api_key.clone(),
    );

    let result = pay_client.order_query(
        arg.out_trade_no.as_deref(),
        arg.transaction_id.as_deref(),
    ).await;

    match result {
        Ok(resp) => {
            // 更新数据库中的订单状态
            if resp.return_code == "SUCCESS" && resp.result_code.as_deref() == Some("SUCCESS") {
                if let Some(ref out_trade_no) = resp.out_trade_no {
                    let _ = WxPayOrdersModel::update_pay_result(
                        out_trade_no,
                        resp.transaction_id.clone(),
                        resp.trade_state.clone(),
                        resp.trade_state_desc.clone(),
                        None,
                        None,
                    ).await;
                }
            }
            ApiResponse::from_result(Ok(serde_json::json!({
                "return_code": resp.return_code,
                "result_code": resp.result_code,
                "out_trade_no": resp.out_trade_no,
                "transaction_id": resp.transaction_id,
                "trade_state": resp.trade_state,
                "trade_state_desc": resp.trade_state_desc,
                "total_fee": resp.total_fee,
            })))
        }
        Err(e) => ApiResponse::<String>::from_result(Err(format!("查询订单失败: {}", e).into())),
    }
}

/// 关闭订单
pub async fn close_order(VJson(arg): VJson<CloseOrderReq>) -> impl IntoResponse {
    let account = crate::model::wechat::model::mwx_accounts::WxAccountsModel::find_by_id(arg.account_id).await;
    let account = match account {
        Ok(Some(a)) => a,
        Ok(None) => return ApiResponse::<String>::from_result(Err("公众号不存在".into())),
        Err(e) => return ApiResponse::<String>::from_result(Err(e)),
    };

    let pay_client = WechatPayClient::new(
        account.app_id.clone(),
        arg.mch_id.clone(),
        arg.api_key.clone(),
    );

    let result = pay_client.close_order(&arg.out_trade_no).await;
    match result {
        Ok(resp) => ApiResponse::from_result(Ok(serde_json::json!({
            "return_code": resp.return_code,
            "result_code": resp.result_code,
        }))),
        Err(e) => ApiResponse::<String>::from_result(Err(format!("关闭订单失败: {}", e).into())),
    }
}

/// 申请退款
pub async fn refund(VJson(arg): VJson<RefundReq>) -> impl IntoResponse {
    let account = crate::model::wechat::model::mwx_accounts::WxAccountsModel::find_by_id(arg.account_id).await;
    let account = match account {
        Ok(Some(a)) => a,
        Ok(None) => return ApiResponse::<String>::from_result(Err("公众号不存在".into())),
        Err(e) => return ApiResponse::<String>::from_result(Err(e)),
    };

    let pay_client = WechatPayClient::new(
        account.app_id.clone(),
        arg.mch_id.clone(),
        arg.api_key.clone(),
    );

    let result = pay_client.refund(
        &arg.out_trade_no,
        &arg.out_refund_no,
        arg.total_fee,
        arg.refund_fee,
        arg.refund_reason.as_deref(),
        arg.notify_url.as_deref(),
    ).await;

    match result {
        Ok(resp) => {
            if resp.return_code == "SUCCESS" && resp.result_code.as_deref() == Some("SUCCESS") {
                // 保存退款记录到数据库
                let refund_arg = WxPayRefundsAddReq {
                    account_id: arg.account_id,
                    out_trade_no: arg.out_trade_no.clone(),
                    out_refund_no: arg.out_refund_no.clone(),
                    transaction_id: resp.transaction_id.clone(),
                    total_fee: arg.total_fee,
                    refund_fee: arg.refund_fee,
                    refund_reason: arg.refund_reason.clone(),
                    refund_account: None,
                };
                let _ = WxPayRefundsModel::add(refund_arg).await;
            }
            ApiResponse::from_result(Ok(serde_json::json!({
                "return_code": resp.return_code,
                "result_code": resp.result_code,
                "refund_id": resp.refund_id,
            })))
        }
        Err(e) => ApiResponse::<String>::from_result(Err(format!("退款失败: {}", e).into())),
    }
}

/// 支付回调通知
pub async fn pay_notify(
    Path(account_id): Path<i64>,
    body: String,
) -> String {
    let account = crate::model::wechat::model::mwx_accounts::WxAccountsModel::find_by_id(account_id).await;
    let _account = match account {
        Ok(Some(a)) => a,
        _ => return WechatPayClient::fail_response("Account not found"),
    };

    let pay_client = WechatPayClient::new(
        _account.app_id.clone(),
        String::new(),
        String::new(),
    );

    let notify = pay_client.parse_pay_notify(&body);
    match notify {
        Ok(n) => {
            if n.return_code == "SUCCESS" && n.result_code.as_deref() == Some("SUCCESS") {
                if let Some(ref out_trade_no) = n.out_trade_no {
                    let _ = WxPayOrdersModel::update_pay_result(
                        out_trade_no,
                        n.transaction_id.clone(),
                        Some("SUCCESS".to_string()),
                        Some("支付成功".to_string()),
                        None,
                        None,
                    ).await;
                }
            }
            WechatPayClient::success_response()
        }
        Err(_) => WechatPayClient::fail_response("Parse notify failed"),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreatePayOrderReq {
    pub account_id: i64,
    pub mch_id: String,
    pub api_key: String,
    pub out_trade_no: String,
    pub openid: Option<String>,
    pub body: String,
    pub total_fee: i64,
    pub fee_type: Option<String>,
    pub trade_type: String,
    pub spbill_create_ip: Option<String>,
    pub notify_url: String,
    pub attach: Option<String>,
    pub time_expire: Option<sea_orm::prelude::DateTimeUtc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct QueryOrderReq {
    pub account_id: i64,
    pub mch_id: String,
    pub api_key: String,
    pub out_trade_no: Option<String>,
    pub transaction_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CloseOrderReq {
    pub account_id: i64,
    pub mch_id: String,
    pub api_key: String,
    pub out_trade_no: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct RefundReq {
    pub account_id: i64,
    pub mch_id: String,
    pub api_key: String,
    pub out_trade_no: String,
    pub out_refund_no: String,
    pub total_fee: i64,
    pub refund_fee: i64,
    pub refund_reason: Option<String>,
    pub notify_url: Option<String>,
}
