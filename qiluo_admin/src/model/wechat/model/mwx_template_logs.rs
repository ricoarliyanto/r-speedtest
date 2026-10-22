pub use super::args::awx_template_logs::*;
pub use super::entity::wx_template_logs::{self, ActiveModel, Model as WxTemplateLogsModel};
use crate::model::prelude::*;
impl WxTemplateLogsModel {
    pub async fn list(
        arg: PageParams,
        search: WxTemplateLogsSearch,
    ) -> Result<ListData<WxTemplateLogsResp>> {
        let page_num = arg.page_num.unwrap_or(1);
        let page_per_size = arg.page_size.unwrap_or(10);
        let db = DB().await;
        let mut rmodel = wx_template_logs::Entity::find();
        if let Some(account_id) = search.account_id {
            rmodel = rmodel.filter(wx_template_logs::Column::AccountId.eq(account_id));
        }
        if let Some(template_id) = search.template_id {
            rmodel = rmodel.filter(wx_template_logs::Column::TemplateId.eq(template_id));
        }
        if let Some(openid) = search.openid {
            rmodel = rmodel.filter(wx_template_logs::Column::Openid.eq(openid));
        }
        if let Some(status) = search.status {
            rmodel = rmodel.filter(wx_template_logs::Column::Status.eq(status));
        }
        let total = rmodel.clone().count(db).await?;
        let paginator = rmodel
            .select_only()
            .column(wx_template_logs::Column::Id)
            .column(wx_template_logs::Column::AccountId)
            .column(wx_template_logs::Column::TemplateId)
            .column(wx_template_logs::Column::Openid)
            .column(wx_template_logs::Column::TemplateData)
            .column(wx_template_logs::Column::Url)
            .column(wx_template_logs::Column::MiniprogramAppid)
            .column(wx_template_logs::Column::MiniprogramPagepath)
            .column(wx_template_logs::Column::MsgId)
            .column(wx_template_logs::Column::Errcode)
            .column(wx_template_logs::Column::Errmsg)
            .column(wx_template_logs::Column::Status)
            .column(wx_template_logs::Column::CreatedAt)
            .into_model::<WxTemplateLogsResp>()
            .paginate(db, page_per_size);
        let total_pages = paginator.num_pages().await?;
        let list = paginator.fetch_page(page_num - 1).await?;
        Ok(ListData { list, total, total_pages, page_num })
    }
}
