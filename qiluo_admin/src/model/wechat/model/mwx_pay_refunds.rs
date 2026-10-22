pub use super::args::awx_pay_refunds::*;
pub use super::entity::wx_pay_refunds::{self, ActiveModel, Model as WxPayRefundsModel};
use crate::model::prelude::*;
impl WxPayRefundsModel {
    pub async fn list(
        arg: PageParams,
        search: WxPayRefundsSearch,
    ) -> Result<ListData<WxPayRefundsResp>> {
        let page_num = arg.page_num.unwrap_or(1);
        let page_per_size = arg.page_size.unwrap_or(10);
        let db = DB().await;
        let mut rmodel = wx_pay_refunds::Entity::find();
        if let Some(account_id) = search.account_id {
            rmodel = rmodel.filter(wx_pay_refunds::Column::AccountId.eq(account_id));
        }
        if let Some(out_trade_no) = search.out_trade_no {
            rmodel = rmodel.filter(wx_pay_refunds::Column::OutTradeNo.eq(out_trade_no));
        }
        if let Some(out_refund_no) = search.out_refund_no {
            rmodel = rmodel.filter(wx_pay_refunds::Column::OutRefundNo.eq(out_refund_no));
        }
        if let Some(refund_status) = search.refund_status {
            rmodel = rmodel.filter(wx_pay_refunds::Column::RefundStatus.eq(refund_status));
        }
        if let Some(status) = search.status {
            rmodel = rmodel.filter(wx_pay_refunds::Column::Status.eq(status));
        }
        let total = rmodel.clone().count(db).await?;
        let paginator = rmodel
            .select_only()
            .column(wx_pay_refunds::Column::Id)
            .column(wx_pay_refunds::Column::AccountId)
            .column(wx_pay_refunds::Column::OutTradeNo)
            .column(wx_pay_refunds::Column::OutRefundNo)
            .column(wx_pay_refunds::Column::TransactionId)
            .column(wx_pay_refunds::Column::RefundId)
            .column(wx_pay_refunds::Column::TotalFee)
            .column(wx_pay_refunds::Column::RefundFee)
            .column(wx_pay_refunds::Column::RefundReason)
            .column(wx_pay_refunds::Column::RefundStatus)
            .column(wx_pay_refunds::Column::RefundAccount)
            .column(wx_pay_refunds::Column::SuccessTime)
            .column(wx_pay_refunds::Column::Status)
            .column(wx_pay_refunds::Column::CreatedAt)
            .column(wx_pay_refunds::Column::UpdatedAt)
            .into_model::<WxPayRefundsResp>()
            .paginate(db, page_per_size);
        let total_pages = paginator.num_pages().await?;
        let list = paginator.fetch_page(page_num - 1).await?;
        Ok(ListData { list, total, total_pages, page_num })
    }
    pub async fn add(arg: WxPayRefundsAddReq) -> Result<String> {
        let db = DB().await;
        let id = GID().await;
        let model = wx_pay_refunds::ActiveModel {
            id: Set(id),
            account_id: Set(arg.account_id),
            out_trade_no: Set(arg.out_trade_no),
            out_refund_no: Set(arg.out_refund_no),
            transaction_id: Set(arg.transaction_id),
            total_fee: Set(arg.total_fee),
            refund_fee: Set(arg.refund_fee),
            refund_reason: Set(arg.refund_reason),
            refund_account: Set(arg.refund_account),
            status: Set(Some(0)),
            ..Default::default()
        };
        let result = wx_pay_refunds::Entity::insert(model).exec(db).await?;
        Ok(format!("{}", result.last_insert_id))
    }
    pub async fn update_refund_result(
        out_refund_no: &str,
        refund_id: Option<String>,
        refund_status: Option<String>,
        success_time: Option<sea_orm::prelude::DateTimeUtc>,
    ) -> Result<String> {
        let db = DB().await;
        let model = wx_pay_refunds::Entity::find()
            .filter(wx_pay_refunds::Column::OutRefundNo.eq(out_refund_no))
            .one(db)
            .await?;
        if let Some(model) = model {
            let mut active_model: wx_pay_refunds::ActiveModel = model.into();
            active_model.refund_id = Set(refund_id);
            active_model.refund_status = Set(refund_status);
            active_model.success_time = Set(success_time);
            active_model.status = Set(Some(1));
            active_model.updated_at = Set(Some(chrono::Utc::now()));
            let _ = active_model.update(db).await?;
            Ok("Success".to_string())
        } else {
            Err("Refund record not found".into())
        }
    }
}

/// 退款请求参数
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct WxPayRefundsAddReq {
    pub account_id: i64,
    pub out_trade_no: String,
    pub out_refund_no: String,
    pub transaction_id: Option<String>,
    pub total_fee: i64,
    pub refund_fee: i64,
    pub refund_reason: Option<String>,
    pub refund_account: Option<String>,
}
