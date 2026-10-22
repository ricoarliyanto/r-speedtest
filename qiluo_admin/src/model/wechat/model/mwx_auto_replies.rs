pub use super::args::awx_auto_replies::*;
pub use super::entity::wx_auto_replies::{self, ActiveModel, Model as WxAutoRepliesModel};
use crate::model::prelude::*;
impl WxAutoRepliesModel {
    pub async fn list(
        arg: PageParams,
        search: WxAutoRepliesSearch,
    ) -> Result<ListData<WxAutoRepliesResp>> {
        let page_num = arg.page_num.unwrap_or(1);
        let page_per_size = arg.page_size.unwrap_or(10);
        let db = DB().await;
        let mut rmodel = wx_auto_replies::Entity::find();
        if let Some(account_id) = search.account_id {
            rmodel = rmodel.filter(wx_auto_replies::Column::AccountId.eq(account_id));
        }
        if let Some(reply_type) = search.reply_type {
            rmodel = rmodel.filter(wx_auto_replies::Column::ReplyType.eq(reply_type));
        }
        if let Some(keyword) = search.keyword {
            rmodel = rmodel.filter(wx_auto_replies::Column::Keyword.eq(keyword));
        }
        if let Some(message_type) = search.message_type {
            rmodel = rmodel.filter(wx_auto_replies::Column::MessageType.eq(message_type));
        }
        if let Some(status) = search.status {
            rmodel = rmodel.filter(wx_auto_replies::Column::Status.eq(status));
        }
        let total = rmodel.clone().count(db).await?;
        let paginator = rmodel
            .select_only()
            .column(wx_auto_replies::Column::Id)
            .column(wx_auto_replies::Column::AccountId)
            .column(wx_auto_replies::Column::ReplyType)
            .column(wx_auto_replies::Column::Keyword)
            .column(wx_auto_replies::Column::MatchType)
            .column(wx_auto_replies::Column::MessageType)
            .column(wx_auto_replies::Column::Content)
            .column(wx_auto_replies::Column::MediaId)
            .column(wx_auto_replies::Column::Title)
            .column(wx_auto_replies::Column::Description)
            .column(wx_auto_replies::Column::PicUrl)
            .column(wx_auto_replies::Column::Url)
            .column(wx_auto_replies::Column::MusicUrl)
            .column(wx_auto_replies::Column::HqMusicUrl)
            .column(wx_auto_replies::Column::ThumbMediaId)
            .column(wx_auto_replies::Column::Status)
            .column(wx_auto_replies::Column::Priority)
            .column(wx_auto_replies::Column::CreatedAt)
            .column(wx_auto_replies::Column::UpdatedAt)
            .into_model::<WxAutoRepliesResp>()
            .paginate(db, page_per_size);
        let total_pages = paginator.num_pages().await?;
        let list = paginator.fetch_page(page_num - 1).await?;
        Ok(ListData { list, total, total_pages, page_num })
    }

    /// 根据 account_id 查找所有启用的自动回复规则（按优先级降序）
    pub async fn find_enabled_by_account(account_id: i64) -> Result<Vec<Self>> {
        let db = DB().await;
        let list = wx_auto_replies::Entity::find()
            .filter(wx_auto_replies::Column::AccountId.eq(account_id))
            .filter(wx_auto_replies::Column::Status.eq(1))
            .order_by_desc(wx_auto_replies::Column::Priority)
            .all(db)
            .await?;
        Ok(list)
    }

    pub async fn add(arg: WxAutoRepliesAdd) -> Result<String> {
        let db = DB().await;
        let id = GID().await;
        let model = wx_auto_replies::ActiveModel {
            id: Set(id),
            account_id: Set(arg.account_id),
            reply_type: Set(arg.reply_type),
            keyword: Set(arg.keyword),
            match_type: Set(arg.match_type),
            message_type: Set(arg.message_type),
            content: Set(arg.content),
            media_id: Set(arg.media_id),
            title: Set(arg.title),
            description: Set(arg.description),
            pic_url: Set(arg.pic_url),
            url: Set(arg.url),
            music_url: Set(arg.music_url),
            hq_music_url: Set(arg.hq_music_url),
            thumb_media_id: Set(arg.thumb_media_id),
            status: Set(arg.status),
            priority: Set(arg.priority),
            created_at: Set(arg.created_at),
            updated_at: Set(arg.updated_at)
        };
        let result = wx_auto_replies::Entity::insert(model).exec(db).await?;
        Ok(format!("Successfully added record with id: {}", result.last_insert_id))
    }
    pub async fn edit(arg: WxAutoRepliesEdit) -> Result<String> {
        let db = DB().await;
        let model = wx_auto_replies::Entity::find_by_id(arg.id).one(db).await?;
        if let Some(model) = model {
            let mut active_model: wx_auto_replies::ActiveModel = model.into();
            active_model.account_id = Set(arg.account_id);
            active_model.reply_type = Set(arg.reply_type);
            active_model.keyword = Set(arg.keyword);
            active_model.match_type = Set(arg.match_type);
            active_model.message_type = Set(arg.message_type);
            active_model.content = Set(arg.content);
            active_model.media_id = Set(arg.media_id);
            active_model.title = Set(arg.title);
            active_model.description = Set(arg.description);
            active_model.pic_url = Set(arg.pic_url);
            active_model.url = Set(arg.url);
            active_model.music_url = Set(arg.music_url);
            active_model.hq_music_url = Set(arg.hq_music_url);
            active_model.thumb_media_id = Set(arg.thumb_media_id);
            active_model.status = Set(arg.status);
            active_model.priority = Set(arg.priority);
            active_model.created_at = Set(arg.created_at);
            active_model.updated_at = Set(arg.updated_at);
            let _ = active_model.update(db).await?;
            Ok("Successfully updated record".to_string())
        } else {
            Err("Record not found".into())
        }
    }
    pub async fn del(arg: WxAutoRepliesDel) -> Result<String> {
        let db = DB().await;
        let result = wx_auto_replies::Entity::delete_by_id(arg.id).exec(db).await?;
        if result.rows_affected > 0 {
            Ok("Success".to_string())
        } else {
            Err("delete failed".into())
        }
    }
}
