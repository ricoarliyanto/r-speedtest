use sea_orm::prelude::DateTimeUtc;

use crate::model::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize, FromQueryResult, Validate)]
pub struct WxPayRefundsResp {
    #[serde(with = "i64_to_string")]
    pub id: i64,
    #[serde(with = "i64_to_string")]
    pub account_id: i64,
    pub out_trade_no: String,
    pub out_refund_no: String,
    pub transaction_id: Option<String>,
    pub refund_id: Option<String>,
    pub total_fee: i64,
    pub refund_fee: i64,
    pub refund_reason: Option<String>,
    pub refund_status: Option<String>,
    pub refund_account: Option<String>,
    pub success_time: Option<DateTimeUtc>,
    pub status: Option<i8>,
    pub created_at: Option<DateTimeUtc>,
    pub updated_at: Option<DateTimeUtc>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct WxPayRefundsSearch {
    pub account_id: Option<i64>,
    pub out_trade_no: Option<String>,
    pub out_refund_no: Option<String>,
    pub refund_status: Option<String>,
    pub status: Option<i8>,
}
