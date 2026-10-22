pub use super::args::awx_pay_orders::*;
pub use super::entity::wx_pay_orders::{self, ActiveModel, Model as WxPayOrdersModel};
use crate::model::prelude::*;
impl WxPayOrdersModel {
    pub async fn find_by_out_trade_no(out_trade_no: &str) -> Result<Option<WxPayOrdersModel>> {
        let db = DB().await;
        let rmodel = wx_pay_orders::Entity::find()
            .filter(wx_pay_orders::Column::OutTradeNo.eq(out_trade_no))
            .one(db)
            .await?;
        Ok(rmodel)
    }
    pub async fn list(
        arg: PageParams,
        search: WxPayOrdersSearch,
    ) -> Result<ListData<WxPayOrdersResp>> {
        let page_num = arg.page_num.unwrap_or(1);
        let page_per_size = arg.page_size.unwrap_or(10);
        let db = DB().await;
        let mut rmodel = wx_pay_orders::Entity::find();
        if let Some(account_id) = search.account_id {
            rmodel = rmodel.filter(wx_pay_orders::Column::AccountId.eq(account_id));
        }
        if let Some(out_trade_no) = search.out_trade_no {
            rmodel = rmodel.filter(wx_pay_orders::Column::OutTradeNo.eq(out_trade_no));
        }
        if let Some(openid) = search.openid {
            rmodel = rmodel.filter(wx_pay_orders::Column::Openid.eq(openid));
        }
        if let Some(trade_type) = search.trade_type {
            rmodel = rmodel.filter(wx_pay_orders::Column::TradeType.eq(trade_type));
        }
        if let Some(trade_state) = search.trade_state {
            rmodel = rmodel.filter(wx_pay_orders::Column::TradeState.eq(trade_state));
        }
        if let Some(status) = search.status {
            rmodel = rmodel.filter(wx_pay_orders::Column::Status.eq(status));
        }
        let total = rmodel.clone().count(db).await?;
        let paginator = rmodel
            .select_only()
            .column(wx_pay_orders::Column::Id)
            .column(wx_pay_orders::Column::AccountId)
            .column(wx_pay_orders::Column::OutTradeNo)
            .column(wx_pay_orders::Column::TransactionId)
            .column(wx_pay_orders::Column::Openid)
            .column(wx_pay_orders::Column::Body)
            .column(wx_pay_orders::Column::TotalFee)
            .column(wx_pay_orders::Column::FeeType)
            .column(wx_pay_orders::Column::TradeType)
            .column(wx_pay_orders::Column::SpbillCreateIp)
            .column(wx_pay_orders::Column::NotifyUrl)
            .column(wx_pay_orders::Column::PrepayId)
            .column(wx_pay_orders::Column::CodeUrl)
            .column(wx_pay_orders::Column::MchId)
            .column(wx_pay_orders::Column::TradeState)
            .column(wx_pay_orders::Column::TradeStateDesc)
            .column(wx_pay_orders::Column::Attach)
            .column(wx_pay_orders::Column::TimeExpire)
            .column(wx_pay_orders::Column::PaidAt)
            .column(wx_pay_orders::Column::Status)
            .column(wx_pay_orders::Column::CreatedAt)
            .column(wx_pay_orders::Column::UpdatedAt)
            .into_model::<WxPayOrdersResp>()
            .paginate(db, page_per_size);
        let total_pages = paginator.num_pages().await?;
        let list = paginator.fetch_page(page_num - 1).await?;
        Ok(ListData { list, total, total_pages, page_num })
    }
    pub async fn add(arg: WxPayOrdersAddReq) -> Result<String> {
        let db = DB().await;
        let id = GID().await;
        let model = wx_pay_orders::ActiveModel {
            id: Set(id),
            account_id: Set(arg.account_id),
            out_trade_no: Set(arg.out_trade_no),
            openid: Set(arg.openid),
            body: Set(arg.body),
            total_fee: Set(arg.total_fee),
            fee_type: Set(arg.fee_type),
            trade_type: Set(arg.trade_type),
            spbill_create_ip: Set(arg.spbill_create_ip),
            notify_url: Set(arg.notify_url),
            mch_id: Set(arg.mch_id),
            attach: Set(arg.attach),
            time_expire: Set(arg.time_expire),
            status: Set(Some(0)),
            ..Default::default()
        };
        let result = wx_pay_orders::Entity::insert(model).exec(db).await?;
        Ok(format!("{}", result.last_insert_id))
    }
    pub async fn update_pay_result(
        out_trade_no: &str,
        transaction_id: Option<String>,
        trade_state: Option<String>,
        trade_state_desc: Option<String>,
        prepay_id: Option<String>,
        code_url: Option<String>,
    ) -> Result<String> {
        let db = DB().await;
        let model = wx_pay_orders::Entity::find()
            .filter(wx_pay_orders::Column::OutTradeNo.eq(out_trade_no))
            .one(db)
            .await?;
        if let Some(model) = model {
            let mut active_model: wx_pay_orders::ActiveModel = model.into();
            active_model.transaction_id = Set(transaction_id);
            active_model.trade_state = Set(trade_state.clone());
            active_model.trade_state_desc = Set(trade_state_desc);
            active_model.prepay_id = Set(prepay_id);
            active_model.code_url = Set(code_url);
            if trade_state.as_deref() == Some("SUCCESS") {
                active_model.paid_at = Set(Some(chrono::Utc::now()));
                active_model.status = Set(Some(1));
            }
            active_model.updated_at = Set(Some(chrono::Utc::now()));
            let _ = active_model.update(db).await?;
            Ok("Success".to_string())
        } else {
            Err("Order not found".into())
        }
    }
}

/// 下单请求参数
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct WxPayOrdersAddReq {
    pub account_id: i64,
    pub out_trade_no: String,
    pub openid: String,
    pub body: String,
    pub total_fee: i64,
    pub fee_type: Option<String>,
    pub trade_type: String,
    pub spbill_create_ip: Option<String>,
    pub notify_url: String,
    pub mch_id: String,
    pub attach: Option<String>,
    pub time_expire: Option<sea_orm::prelude::DateTimeUtc>,
}
