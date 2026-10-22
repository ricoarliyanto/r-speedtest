pub use super::args::awx_accounts::*;
pub use super::entity::wx_accounts::{self, ActiveModel, Model as WxAccountsModel};
use crate::model::prelude::*;
impl WxAccountsModel {
    pub async fn find_by_id(id: i64) -> Result<Option<WxAccountsModel>> {
        let db = DB().await;
        let rmodel = wx_accounts::Entity::find_by_id(id).one(db).await?;
        Ok(rmodel)
    }
    pub async fn find_by_origin_id(origin_id: &str) -> Result<Option<WxAccountsModel>> {
        let db = DB().await;
        let rmodel = wx_accounts::Entity::find()
            .filter(wx_accounts::Column::OriginalId.eq(origin_id))
            .one(db)
            .await?;
        Ok(rmodel)
    }
    pub async fn list(
        arg: PageParams,
        search: WxAccountsSearch,
    ) -> Result<ListData<WxAccountsResp>> {
        let page_num = arg.page_num.unwrap_or(1);
        let page_per_size = arg.page_size.unwrap_or(10);
        let db = DB().await;
        let mut rmodel = wx_accounts::Entity::find();
        if let Some(wechat_id) = search.wechat_id {
            rmodel = rmodel.filter(wx_accounts::Column::WechatId.eq(wechat_id));
        }
        if let Some(original_id) = search.original_id {
            rmodel = rmodel.filter(wx_accounts::Column::OriginalId.eq(original_id));
        }
        if let Some(account_name) = search.account_name {
            rmodel = rmodel.filter(wx_accounts::Column::AccountName.eq(account_name));
        }
        if let Some(status) = search.status {
            rmodel = rmodel.filter(wx_accounts::Column::Status.eq(status));
        }
        if let Some(app_id) = search.app_id {
            rmodel = rmodel.filter(wx_accounts::Column::AppId.eq(app_id));
        }
        if let Some(account_type) = search.account_type {
            rmodel = rmodel.filter(wx_accounts::Column::AccountType.eq(account_type));
        }
        let total = rmodel.clone().count(db).await?;
        let paginator = rmodel
            .select_only()
            .column(wx_accounts::Column::Id)
            .column(wx_accounts::Column::AppId)
            .column(wx_accounts::Column::AppSecret)
            .column(wx_accounts::Column::AccountName)
            .column(wx_accounts::Column::AccountType)
            .column(wx_accounts::Column::OriginalId)
            .column(wx_accounts::Column::WechatId)
            .column(wx_accounts::Column::Status)
            .column(wx_accounts::Column::MessageMode)
            .column(wx_accounts::Column::AccessToken)
            .column(wx_accounts::Column::TokenExpiresAt)
            .column(wx_accounts::Column::ServerUrl)
            .column(wx_accounts::Column::Token)
            .column(wx_accounts::Column::EncodingAesKey)
            .column(wx_accounts::Column::CreatedAt)
            .column(wx_accounts::Column::UpdatedAt)
            .into_model::<WxAccountsResp>()
            .paginate(db, page_per_size);
        let total_pages = paginator.num_pages().await?;
        let list = paginator.fetch_page(page_num - 1).await?;
        Ok(ListData { list, total, total_pages, page_num })
    }
    pub async fn add(arg: WxAccountsAdd) -> Result<String> {
        let db = DB().await;
        let id = GID().await;
        let model = wx_accounts::ActiveModel {
            id: Set(id),
            app_id: Set(arg.app_id),
            app_secret: Set(arg.app_secret),
            account_name: Set(arg.account_name),
            account_type: Set(arg.account_type),
            original_id: Set(arg.original_id),
            wechat_id: Set(arg.wechat_id),
            status: Set(arg.status),
            message_mode: Set(arg.message_mode),
            access_token: Set(arg.access_token),
            token_expires_at: Set(arg.token_expires_at),
            server_url: Set(arg.server_url),
            token: Set(arg.token),
            encoding_aes_key: Set(arg.encoding_aes_key),
            created_at: Set(arg.created_at),
            updated_at: Set(arg.updated_at)
        };
        let result = wx_accounts::Entity::insert(model).exec(db).await?;
        Ok(format!("Successfully added record with id: {}", result.last_insert_id))
    }
    pub async fn edit(arg: WxAccountsEdit) -> Result<String> {
        let db = DB().await;
        let model = wx_accounts::Entity::find_by_id(arg.id).one(db).await?;
        if let Some(model) = model {
            let mut active_model: wx_accounts::ActiveModel = model.into();
            active_model.account_name = Set(arg.account_name);
            active_model.wechat_id = Set(arg.wechat_id);
            active_model.original_id = Set(arg.original_id);
            active_model.id = Set(arg.id);
            active_model.status = Set(arg.status);
            active_model.app_secret = Set(arg.app_secret);
            active_model.app_id = Set(arg.app_id);
            active_model.token = Set(arg.token);
            active_model.token_expires_at = Set(arg.token_expires_at);
            active_model.created_at = Set(arg.created_at);
            active_model.account_type = Set(arg.account_type);
            active_model.encoding_aes_key = Set(arg.encoding_aes_key);
            active_model.updated_at = Set(arg.updated_at);
            active_model.server_url = Set(arg.server_url);
            active_model.access_token = Set(arg.access_token);
            active_model.message_mode = Set(arg.message_mode);
            let _ = active_model.update(db).await?;
            Ok("Successfully updated record".to_string())
        } else {
            Err("Record not found".into())
        }
    }
    pub async fn del(arg: WxAccountsDel) -> Result<String> {
        let db = DB().await;
        let result = wx_accounts::Entity::delete_by_id(arg.id).exec(db).await?;
        if result.rows_affected > 0 {
            Ok("Success".to_string())
        } else {
            Err("delete failed".into())
        }
    }
}
