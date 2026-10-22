use serde::{Deserialize, Serialize};

/// 统一下单请求 (V2 XML接口)
#[derive(Debug, Clone, Serialize)]
#[serde(rename = "xml")]
pub struct UnifiedOrderRequest {
    pub appid: String,
    pub mch_id: String,
    pub nonce_str: String,
    pub sign: String,
    pub sign_type: String,
    pub body: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach: Option<String>,
    pub out_trade_no: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_type: Option<String>,
    pub total_fee: i64,
    pub spbill_create_ip: String,
    pub notify_url: String,
    pub trade_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_expire: Option<String>,
}

/// 统一下单响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "xml")]
pub struct UnifiedOrderResponse {
    pub return_code: String,
    pub return_msg: Option<String>,
    pub result_code: Option<String>,
    pub err_code: Option<String>,
    pub err_code_des: Option<String>,
    pub appid: Option<String>,
    pub mch_id: Option<String>,
    pub nonce_str: Option<String>,
    pub sign: Option<String>,
    pub prepay_id: Option<String>,
    pub trade_type: Option<String>,
    pub code_url: Option<String>,
    pub mweb_url: Option<String>,
}

/// 查询订单请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename = "xml")]
pub struct OrderQueryRequest {
    pub appid: String,
    pub mch_id: String,
    pub nonce_str: String,
    pub sign: String,
    pub sign_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_trade_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
}

/// 查询订单响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "xml")]
pub struct OrderQueryResponse {
    pub return_code: String,
    pub return_msg: Option<String>,
    pub result_code: Option<String>,
    pub err_code: Option<String>,
    pub err_code_des: Option<String>,
    pub appid: Option<String>,
    pub mch_id: Option<String>,
    pub out_trade_no: Option<String>,
    pub transaction_id: Option<String>,
    pub trade_state: Option<String>,
    pub trade_state_desc: Option<String>,
    pub total_fee: Option<i64>,
    pub cash_fee: Option<i64>,
    pub time_end: Option<String>,
    pub openid: Option<String>,
}

/// 申请退款请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename = "xml")]
pub struct RefundRequest {
    pub appid: String,
    pub mch_id: String,
    pub nonce_str: String,
    pub sign: String,
    pub sign_type: String,
    pub out_trade_no: String,
    pub out_refund_no: String,
    pub total_fee: i64,
    pub refund_fee: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_fee_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_desc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_url: Option<String>,
}

/// 申请退款响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "xml")]
pub struct RefundResponse {
    pub return_code: String,
    pub return_msg: Option<String>,
    pub result_code: Option<String>,
    pub err_code: Option<String>,
    pub err_code_des: Option<String>,
    pub appid: Option<String>,
    pub mch_id: Option<String>,
    pub nonce_str: Option<String>,
    pub sign: Option<String>,
    pub out_trade_no: Option<String>,
    pub out_refund_no: Option<String>,
    pub transaction_id: Option<String>,
    pub refund_id: Option<String>,
    pub refund_fee: Option<i64>,
}

/// 关闭订单请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename = "xml")]
pub struct CloseOrderRequest {
    pub appid: String,
    pub mch_id: String,
    pub nonce_str: String,
    pub sign: String,
    pub sign_type: String,
    pub out_trade_no: String,
}

/// 关闭订单响应
#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "xml")]
pub struct CloseOrderResponse {
    pub return_code: String,
    pub return_msg: Option<String>,
    pub result_code: Option<String>,
    pub err_code: Option<String>,
    pub err_code_des: Option<String>,
}

/// 支付回调通知
#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "xml")]
pub struct PayNotify {
    pub return_code: String,
    pub return_msg: Option<String>,
    pub result_code: Option<String>,
    pub appid: Option<String>,
    pub mch_id: Option<String>,
    pub nonce_str: Option<String>,
    pub sign: Option<String>,
    pub out_trade_no: Option<String>,
    pub transaction_id: Option<String>,
    pub total_fee: Option<i64>,
    pub cash_fee: Option<i64>,
    pub fee_type: Option<String>,
    pub openid: Option<String>,
    pub trade_type: Option<String>,
    pub time_end: Option<String>,
    pub attach: Option<String>,
}

/// 退款回调通知
#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "xml")]
pub struct RefundNotify {
    pub return_code: String,
    pub return_msg: Option<String>,
    pub result_code: Option<String>,
    pub appid: Option<String>,
    pub mch_id: Option<String>,
    pub nonce_str: Option<String>,
    pub out_trade_no: Option<String>,
    pub out_refund_no: Option<String>,
    pub transaction_id: Option<String>,
    pub refund_id: Option<String>,
    pub refund_fee: Option<i64>,
    pub refund_status: Option<String>,
    pub success_time: Option<String>,
}

/// 支付回调响应
#[derive(Debug, Clone, Serialize)]
#[serde(rename = "xml")]
pub struct NotifyResponse {
    pub return_code: String,
    pub return_msg: Option<String>,
}

impl NotifyResponse {
    pub fn success() -> Self {
        Self {
            return_code: "SUCCESS".to_string(),
            return_msg: Some("OK".to_string()),
        }
    }
    pub fn fail(msg: &str) -> Self {
        Self {
            return_code: "FAIL".to_string(),
            return_msg: Some(msg.to_string()),
        }
    }
}

/// JSAPI 下单参数 (返回给前端的参数)
#[derive(Debug, Clone, Serialize)]
pub struct JsapiPayParams {
    pub appid: String,
    pub partnerid: String,
    pub prepayid: String,
    pub package: String,
    pub noncestr: String,
    pub timestamp: String,
    pub sign: String,
}

/// 交易类型枚举
pub enum TradeType {
    /// JSAPI 公众号支付
    Jsapi,
    /// NATIVE 扫码支付
    Native,
    /// APP 支付
    App,
    /// H5 支付
    Mweb,
}

impl TradeType {
    pub fn as_str(&self) -> &str {
        match self {
            TradeType::Jsapi => "JSAPI",
            TradeType::Native => "NATIVE",
            TradeType::App => "APP",
            TradeType::Mweb => "MWEB",
        }
    }
}
