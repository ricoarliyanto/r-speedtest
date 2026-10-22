use sea_orm::prelude::DateTimeUtc;

use crate::model::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize, FromQueryResult, Validate)]
pub struct WxPayOrdersResp {
    #[serde(with = "i64_to_string")]
    pub id: i64,
    #[serde(with = "i64_to_string")]
    pub account_id: i64,
    pub out_trade_no: String,
    pub transaction_id: Option<String>,
    pub openid: String,
    pub body: String,
    pub total_fee: i64,
    pub fee_type: Option<String>,
    pub trade_type: String,
    pub spbill_create_ip: Option<String>,
    pub notify_url: String,
    pub prepay_id: Option<String>,
    pub code_url: Option<String>,
    pub mch_id: String,
    pub trade_state: Option<String>,
    pub trade_state_desc: Option<String>,
    pub attach: Option<String>,
    pub time_expire: Option<DateTimeUtc>,
    pub paid_at: Option<DateTimeUtc>,
    pub status: Option<i8>,
    pub created_at: Option<DateTimeUtc>,
    pub updated_at: Option<DateTimeUtc>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct WxPayOrdersSearch {
    pub account_id: Option<i64>,
    pub out_trade_no: Option<String>,
    pub openid: Option<String>,
    pub trade_type: Option<String>,
    pub trade_state: Option<String>,
    pub status: Option<i8>,
}
