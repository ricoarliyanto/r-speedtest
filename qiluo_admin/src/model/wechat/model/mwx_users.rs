pub use super::args::awx_users::*;
pub use super::entity::wx_users::{self, ActiveModel, Model as WxUsersModel};
use crate::model::prelude::*;
impl WxUsersModel {
    pub async fn find_by_openid(account_id: i64, openid: &str) -> Result<Option<WxUsersModel>> {
        let db = DB().await;
        let rmodel = wx_users::Entity::find()
            .filter(wx_users::Column::AccountId.eq(account_id))
            .filter(wx_users::Column::Openid.eq(openid))
            .one(db)
            .await?;
        Ok(rmodel)
    }
    pub async fn list(
        arg: PageParams,
        search: WxUsersSearch,
    ) -> Result<ListData<WxUsersResp>> {
        let page_num = arg.page_num.unwrap_or(1);
        let page_per_size = arg.page_size.unwrap_or(10);
        let db = DB().await;
        let mut rmodel = wx_users::Entity::find();
        if let Some(account_id) = search.account_id {
            rmodel = rmodel.filter(wx_users::Column::AccountId.eq(account_id));
        }
        if let Some(openid) = search.openid {
            rmodel = rmodel.filter(wx_users::Column::Openid.eq(openid));
        }
        if let Some(nickname) = search.nickname {
            rmodel = rmodel.filter(wx_users::Column::Nickname.eq(nickname));
        }
        if let Some(sex) = search.sex {
            rmodel = rmodel.filter(wx_users::Column::Sex.eq(sex));
        }
        if let Some(subscribe_status) = search.subscribe_status {
            rmodel = rmodel.filter(wx_users::Column::SubscribeStatus.eq(subscribe_status));
        }
        let total = rmodel.clone().count(db).await?;
        let paginator = rmodel
            .select_only()
            .column(wx_users::Column::Id)
            .column(wx_users::Column::AccountId)
            .column(wx_users::Column::Openid)
            .column(wx_users::Column::Unionid)
            .column(wx_users::Column::Nickname)
            .column(wx_users::Column::Sex)
            .column(wx_users::Column::City)
            .column(wx_users::Column::Country)
            .column(wx_users::Column::Province)
            .column(wx_users::Column::Language)
            .column(wx_users::Column::Headimgurl)
            .column(wx_users::Column::SubscribeTime)
            .column(wx_users::Column::UnsubscribeTime)
            .column(wx_users::Column::SubscribeStatus)
            .column(wx_users::Column::Remark)
            .column(wx_users::Column::SubscribeScene)
            .column(wx_users::Column::QrScene)
            .column(wx_users::Column::QrSceneStr)
            .column(wx_users::Column::LastInteractTime)
            .column(wx_users::Column::MessageCount)
            .column(wx_users::Column::CreatedAt)
            .column(wx_users::Column::UpdatedAt)
            .into_model::<WxUsersResp>()
            .paginate(db, page_per_size);
        let total_pages = paginator.num_pages().await?;
        let list = paginator.fetch_page(page_num - 1).await?;
        Ok(ListData { list, total, total_pages, page_num })
    }
    pub async fn add(arg: WxUsersAdd) -> Result<String> {
        let db = DB().await;
        let id = GID().await;
        let model = wx_users::ActiveModel {
            id: Set(id),
            account_id: Set(arg.account_id),
            openid: Set(arg.openid),
            unionid: Set(arg.unionid),
            nickname: Set(arg.nickname),
            sex: Set(arg.sex),
            city: Set(arg.city),
            country: Set(arg.country),
            province: Set(arg.province),
            language: Set(arg.language),
            headimgurl: Set(arg.headimgurl),
            subscribe_time: Set(arg.subscribe_time),
            unsubscribe_time: Set(arg.unsubscribe_time),
            subscribe_status: Set(arg.subscribe_status),
            remark: Set(arg.remark),
            subscribe_scene: Set(arg.subscribe_scene),
            qr_scene: Set(arg.qr_scene),
            qr_scene_str: Set(arg.qr_scene_str),
            last_interact_time: Set(arg.last_interact_time),
            message_count: Set(arg.message_count),
            created_at: Set(arg.created_at),
            updated_at: Set(arg.updated_at)
        };
        let result = wx_users::Entity::insert(model).exec(db).await?;
        Ok(format!("Successfully added record with id: {}", result.last_insert_id))
    }
    pub async fn edit(arg: WxUsersEdit) -> Result<String> {
        let db = DB().await;
        let model = wx_users::Entity::find_by_id(arg.id).one(db).await?;
        if let Some(model) = model {
            let mut active_model: wx_users::ActiveModel = model.into();
            active_model.account_id = Set(arg.account_id);
            active_model.openid = Set(arg.openid);
            active_model.unionid = Set(arg.unionid);
            active_model.nickname = Set(arg.nickname);
            active_model.sex = Set(arg.sex);
            active_model.city = Set(arg.city);
            active_model.country = Set(arg.country);
            active_model.province = Set(arg.province);
            active_model.language = Set(arg.language);
            active_model.headimgurl = Set(arg.headimgurl);
            active_model.subscribe_time = Set(arg.subscribe_time);
            active_model.unsubscribe_time = Set(arg.unsubscribe_time);
            active_model.subscribe_status = Set(arg.subscribe_status);
            active_model.remark = Set(arg.remark);
            active_model.subscribe_scene = Set(arg.subscribe_scene);
            active_model.qr_scene = Set(arg.qr_scene);
            active_model.qr_scene_str = Set(arg.qr_scene_str);
            active_model.last_interact_time = Set(arg.last_interact_time);
            active_model.message_count = Set(arg.message_count);
            active_model.created_at = Set(arg.created_at);
            active_model.updated_at = Set(arg.updated_at);
            let _ = active_model.update(db).await?;
            Ok("Successfully updated record".to_string())
        } else {
            Err("Record not found".into())
        }
    }
    pub async fn del(arg: WxUsersDel) -> Result<String> {
        let db = DB().await;
        let result = wx_users::Entity::delete_by_id(arg.id).exec(db).await?;
        if result.rows_affected > 0 {
            Ok("Success".to_string())
        } else {
            Err("delete failed".into())
        }
    }
}
