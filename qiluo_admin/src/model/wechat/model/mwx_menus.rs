pub use super::args::awx_menus::*;
pub use super::entity::wx_menus::{self, ActiveModel, Model as WxMenusModel};
use crate::model::prelude::*;
impl WxMenusModel {
    pub async fn list(
        arg: PageParams,
        search: WxMenusSearch,
    ) -> Result<ListData<WxMenusResp>> {
        let page_num = arg.page_num.unwrap_or(1);
        let page_per_size = arg.page_size.unwrap_or(10);
        let db = DB().await;
        let mut rmodel = wx_menus::Entity::find();
        if let Some(account_id) = search.account_id {
            rmodel = rmodel.filter(wx_menus::Column::AccountId.eq(account_id));
        }
        if let Some(menu_name) = search.menu_name {
            rmodel = rmodel.filter(wx_menus::Column::MenuName.eq(menu_name));
        }
        if let Some(menu_type) = search.menu_type {
            rmodel = rmodel.filter(wx_menus::Column::MenuType.eq(menu_type));
        }
        if let Some(status) = search.status {
            rmodel = rmodel.filter(wx_menus::Column::Status.eq(status));
        }
        let total = rmodel.clone().count(db).await?;
        let paginator = rmodel
            .select_only()
            .column(wx_menus::Column::Id)
            .column(wx_menus::Column::AccountId)
            .column(wx_menus::Column::ParentId)
            .column(wx_menus::Column::MenuName)
            .column(wx_menus::Column::MenuType)
            .column(wx_menus::Column::MenuKey)
            .column(wx_menus::Column::Url)
            .column(wx_menus::Column::MediaId)
            .column(wx_menus::Column::Appid)
            .column(wx_menus::Column::Pagepath)
            .column(wx_menus::Column::ArticleId)
            .column(wx_menus::Column::SortOrder)
            .column(wx_menus::Column::Status)
            .column(wx_menus::Column::CreatedAt)
            .column(wx_menus::Column::UpdatedAt)
            .into_model::<WxMenusResp>()
            .paginate(db, page_per_size);
        let total_pages = paginator.num_pages().await?;
        let list = paginator.fetch_page(page_num - 1).await?;
        Ok(ListData { list, total, total_pages, page_num })
    }
    pub async fn add(arg: WxMenusAdd) -> Result<String> {
        let db = DB().await;
        let id = GID().await;
        let model = wx_menus::ActiveModel {
            id: Set(id),
            account_id: Set(arg.account_id),
            parent_id: Set(arg.parent_id),
            menu_name: Set(arg.menu_name),
            menu_type: Set(arg.menu_type),
            menu_key: Set(arg.menu_key),
            url: Set(arg.url),
            media_id: Set(arg.media_id),
            appid: Set(arg.appid),
            pagepath: Set(arg.pagepath),
            article_id: Set(arg.article_id),
            sort_order: Set(arg.sort_order),
            status: Set(arg.status),
            created_at: Set(arg.created_at),
            updated_at: Set(arg.updated_at)
        };
        let result = wx_menus::Entity::insert(model).exec(db).await?;
        Ok(format!("Successfully added record with id: {}", result.last_insert_id))
    }
    pub async fn edit(arg: WxMenusEdit) -> Result<String> {
        let db = DB().await;
        let model = wx_menus::Entity::find_by_id(arg.id).one(db).await?;
        if let Some(model) = model {
            let mut active_model: wx_menus::ActiveModel = model.into();
            active_model.account_id = Set(arg.account_id);
            active_model.parent_id = Set(arg.parent_id);
            active_model.menu_name = Set(arg.menu_name);
            active_model.menu_type = Set(arg.menu_type);
            active_model.menu_key = Set(arg.menu_key);
            active_model.url = Set(arg.url);
            active_model.media_id = Set(arg.media_id);
            active_model.appid = Set(arg.appid);
            active_model.pagepath = Set(arg.pagepath);
            active_model.article_id = Set(arg.article_id);
            active_model.sort_order = Set(arg.sort_order);
            active_model.status = Set(arg.status);
            active_model.created_at = Set(arg.created_at);
            active_model.updated_at = Set(arg.updated_at);
            let _ = active_model.update(db).await?;
            Ok("Successfully updated record".to_string())
        } else {
            Err("Record not found".into())
        }
    }
    pub async fn del(arg: WxMenusDel) -> Result<String> {
        let db = DB().await;
        let result = wx_menus::Entity::delete_by_id(arg.id).exec(db).await?;
        if result.rows_affected > 0 {
            Ok("Success".to_string())
        } else {
            Err("delete failed".into())
        }
    }

    /// 获取指定公众号的所有菜单（用于发布到微信）
    pub async fn get_all_by_account(account_id: i64) -> Result<Vec<(i64, Option<i64>, String, String, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, i32, i8)>> {
        let db = DB().await;
        let list = wx_menus::Entity::find()
            .filter(wx_menus::Column::AccountId.eq(account_id))
            .order_by_asc(wx_menus::Column::SortOrder)
            .select_only()
            .column(wx_menus::Column::Id)
            .column(wx_menus::Column::ParentId)
            .column(wx_menus::Column::MenuName)
            .column(wx_menus::Column::MenuType)
            .column(wx_menus::Column::MenuKey)
            .column(wx_menus::Column::Url)
            .column(wx_menus::Column::MediaId)
            .column(wx_menus::Column::Appid)
            .column(wx_menus::Column::Pagepath)
            .column(wx_menus::Column::SortOrder)
            .column(wx_menus::Column::Status)
            .into_tuple::<(i64, Option<i64>, String, String, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, i32, i8)>()
            .all(db)
            .await?;
        Ok(list)
    }

    /// 删除指定公众号的所有菜单（用于同步前清空旧数据）
    pub async fn delete_by_account(account_id: i64) -> Result<String> {
        let db = DB().await;
        let result = wx_menus::Entity::delete_many()
            .filter(wx_menus::Column::AccountId.eq(account_id))
            .exec(db)
            .await?;
        Ok(format!("Deleted {} menu items", result.rows_affected))
    }
}
