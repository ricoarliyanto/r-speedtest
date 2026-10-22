pub use super::args::awx_templates::*;
pub use super::entity::wx_templates::{self, ActiveModel, Model as WxTemplatesModel};
use crate::model::prelude::*;
impl WxTemplatesModel {
    pub async fn find_by_id(id: i64) -> Result<Option<WxTemplatesModel>> {
        let db = DB().await;
        let rmodel = wx_templates::Entity::find_by_id(id).one(db).await?;
        Ok(rmodel)
    }
    pub async fn list(
        arg: PageParams,
        search: WxTemplatesSearch,
    ) -> Result<ListData<WxTemplatesResp>> {
        let page_num = arg.page_num.unwrap_or(1);
        let page_per_size = arg.page_size.unwrap_or(10);
        let db = DB().await;
        let mut rmodel = wx_templates::Entity::find();
        if let Some(account_id) = search.account_id {
            rmodel = rmodel.filter(wx_templates::Column::AccountId.eq(account_id));
        }
        if let Some(template_id) = search.template_id {
            rmodel = rmodel.filter(wx_templates::Column::TemplateId.eq(template_id));
        }
        if let Some(title) = search.title {
            rmodel = rmodel.filter(wx_templates::Column::Title.contains(title));
        }
        if let Some(status) = search.status {
            rmodel = rmodel.filter(wx_templates::Column::Status.eq(status));
        }
        let total = rmodel.clone().count(db).await?;
        let paginator = rmodel
            .select_only()
            .column(wx_templates::Column::Id)
            .column(wx_templates::Column::AccountId)
            .column(wx_templates::Column::TemplateId)
            .column(wx_templates::Column::Title)
            .column(wx_templates::Column::Industry)
            .column(wx_templates::Column::Content)
            .column(wx_templates::Column::Example)
            .column(wx_templates::Column::TemplateType)
            .column(wx_templates::Column::Status)
            .column(wx_templates::Column::CreatedAt)
            .column(wx_templates::Column::UpdatedAt)
            .into_model::<WxTemplatesResp>()
            .paginate(db, page_per_size);
        let total_pages = paginator.num_pages().await?;
        let list = paginator.fetch_page(page_num - 1).await?;
        Ok(ListData { list, total, total_pages, page_num })
    }
    pub async fn add(arg: WxTemplatesAdd) -> Result<String> {
        let db = DB().await;
        let id = GID().await;
        let model = wx_templates::ActiveModel {
            id: Set(id),
            account_id: Set(arg.account_id),
            template_id: Set(arg.template_id),
            title: Set(arg.title),
            industry: Set(arg.industry),
            content: Set(arg.content),
            example: Set(arg.example),
            template_type: Set(arg.template_type),
            status: Set(arg.status),
            created_at: Set(arg.created_at),
            updated_at: Set(arg.updated_at)
        };
        let result = wx_templates::Entity::insert(model).exec(db).await?;
        Ok(format!("Successfully added record with id: {}", result.last_insert_id))
    }
    pub async fn edit(arg: WxTemplatesEdit) -> Result<String> {
        let db = DB().await;
        let model = wx_templates::Entity::find_by_id(arg.id).one(db).await?;
        if let Some(model) = model {
            let mut active_model: wx_templates::ActiveModel = model.into();
            active_model.id = Set(arg.id);
            active_model.account_id = Set(arg.account_id);
            active_model.template_id = Set(arg.template_id);
            active_model.title = Set(arg.title);
            active_model.industry = Set(arg.industry);
            active_model.content = Set(arg.content);
            active_model.example = Set(arg.example);
            active_model.template_type = Set(arg.template_type);
            active_model.status = Set(arg.status);
            active_model.created_at = Set(arg.created_at);
            active_model.updated_at = Set(arg.updated_at);
            let _ = active_model.update(db).await?;
            Ok("Successfully updated record".to_string())
        } else {
            Err("Record not found".into())
        }
    }
    pub async fn del(arg: WxTemplatesDel) -> Result<String> {
        let db = DB().await;
        let result = wx_templates::Entity::delete_by_id(arg.id).exec(db).await?;
        if result.rows_affected > 0 {
            Ok("Success".to_string())
        } else {
            Err("delete failed".into())
        }
    }
}
