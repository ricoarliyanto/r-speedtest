pub use super::args::awx_messages::*;
pub use super::entity::wx_messages::{self, ActiveModel, Model as WxMessagesModel};
use crate::model::prelude::*;
impl WxMessagesModel {
    pub async fn list(
        arg: PageParams,
        search: WxMessagesSearch,
    ) -> Result<ListData<WxMessagesResp>> {
        let page_num = arg.page_num.unwrap_or(1);
        let page_per_size = arg.page_size.unwrap_or(10);
        let db = DB().await;
        let mut rmodel = wx_messages::Entity::find();
        if let Some(account_id) = search.account_id {
            rmodel = rmodel.filter(wx_messages::Column::AccountId.eq(account_id));
        }
        if let Some(openid) = search.openid {
            rmodel = rmodel.filter(wx_messages::Column::Openid.eq(openid));
        }
        if let Some(msg_type) = search.msg_type {
            rmodel = rmodel.filter(wx_messages::Column::MsgType.eq(msg_type));
        }
        if let Some(direction) = search.direction {
            rmodel = rmodel.filter(wx_messages::Column::Direction.eq(direction));
        }
        let total = rmodel.clone().count(db).await?;
        let paginator = rmodel
            .select_only()
            .column(wx_messages::Column::Id)
            .column(wx_messages::Column::AccountId)
            .column(wx_messages::Column::Openid)
            .column(wx_messages::Column::MsgId)
            .column(wx_messages::Column::MsgType)
            .column(wx_messages::Column::Direction)
            .column(wx_messages::Column::Content)
            .column(wx_messages::Column::MediaId)
            .column(wx_messages::Column::PicUrl)
            .column(wx_messages::Column::VoiceFormat)
            .column(wx_messages::Column::Recognition)
            .column(wx_messages::Column::ThumbMediaId)
            .column(wx_messages::Column::MsgTitle)
            .column(wx_messages::Column::MsgDescription)
            .column(wx_messages::Column::LinkUrl)
            .column(wx_messages::Column::EventType)
            .column(wx_messages::Column::EventKey)
            .column(wx_messages::Column::ReplyMsgId)
            .column(wx_messages::Column::IsAutoReply)
            .column(wx_messages::Column::CreatedAt)
            .into_model::<WxMessagesResp>()
            .paginate(db, page_per_size);
        let total_pages = paginator.num_pages().await?;
        let list = paginator.fetch_page(page_num - 1).await?;
        Ok(ListData { list, total, total_pages, page_num })
    }
    pub async fn add(arg: WxMessagesAdd) -> Result<String> {
        let db = DB().await;
        let id = GID().await;
        let model = wx_messages::ActiveModel {
            id: Set(id),
            account_id: Set(arg.account_id),
            openid: Set(arg.openid),
            msg_id: Set(arg.msg_id),
            msg_type: Set(arg.msg_type),
            direction: Set(arg.direction),
            content: Set(arg.content),
            media_id: Set(arg.media_id),
            pic_url: Set(arg.pic_url),
            voice_format: Set(arg.voice_format),
            recognition: Set(arg.recognition),
            thumb_media_id: Set(arg.thumb_media_id),
            msg_title: Set(arg.msg_title),
            msg_description: Set(arg.msg_description),
            link_url: Set(arg.link_url),
            event_type: Set(arg.event_type),
            event_key: Set(arg.event_key),
            reply_msg_id: Set(arg.reply_msg_id),
            is_auto_reply: Set(arg.is_auto_reply),
            created_at: Set(arg.created_at)
        };
        let result = wx_messages::Entity::insert(model).exec(db).await?;
        Ok(format!("Successfully added record with id: {}", result.last_insert_id))
    }
    pub async fn edit(arg: WxMessagesEdit) -> Result<String> {
        let db = DB().await;
        let model = wx_messages::Entity::find_by_id(arg.id).one(db).await?;
        if let Some(model) = model {
            let mut active_model: wx_messages::ActiveModel = model.into();
            active_model.account_id = Set(arg.account_id);
            active_model.openid = Set(arg.openid);
            active_model.msg_id = Set(arg.msg_id);
            active_model.msg_type = Set(arg.msg_type);
            active_model.direction = Set(arg.direction);
            active_model.content = Set(arg.content);
            active_model.media_id = Set(arg.media_id);
            active_model.pic_url = Set(arg.pic_url);
            active_model.voice_format = Set(arg.voice_format);
            active_model.recognition = Set(arg.recognition);
            active_model.thumb_media_id = Set(arg.thumb_media_id);
            active_model.msg_title = Set(arg.msg_title);
            active_model.msg_description = Set(arg.msg_description);
            active_model.link_url = Set(arg.link_url);
            active_model.event_type = Set(arg.event_type);
            active_model.event_key = Set(arg.event_key);
            active_model.reply_msg_id = Set(arg.reply_msg_id);
            active_model.is_auto_reply = Set(arg.is_auto_reply);
            active_model.created_at = Set(arg.created_at);
            let _ = active_model.update(db).await?;
            Ok("Successfully updated record".to_string())
        } else {
            Err("Record not found".into())
        }
    }
    pub async fn del(arg: WxMessagesDel) -> Result<String> {
        let db = DB().await;
        let result = wx_messages::Entity::delete_by_id(arg.id).exec(db).await?;
        if result.rows_affected > 0 {
            Ok("Success".to_string())
        } else {
            Err("delete failed".into())
        }
    }
    /// 获取某个用户的会话消息列表（按时间正序）
    pub async fn get_conversation(account_id: i64, openid: &str) -> Result<Vec<WxMessagesResp>> {
        let db = DB().await;
        let list = wx_messages::Entity::find()
            .filter(wx_messages::Column::AccountId.eq(account_id))
            .filter(wx_messages::Column::Openid.eq(openid))
            .order_by_asc(wx_messages::Column::CreatedAt)
            .select_only()
            .column(wx_messages::Column::Id)
            .column(wx_messages::Column::AccountId)
            .column(wx_messages::Column::Openid)
            .column(wx_messages::Column::MsgId)
            .column(wx_messages::Column::MsgType)
            .column(wx_messages::Column::Direction)
            .column(wx_messages::Column::Content)
            .column(wx_messages::Column::MediaId)
            .column(wx_messages::Column::PicUrl)
            .column(wx_messages::Column::VoiceFormat)
            .column(wx_messages::Column::Recognition)
            .column(wx_messages::Column::ThumbMediaId)
            .column(wx_messages::Column::MsgTitle)
            .column(wx_messages::Column::MsgDescription)
            .column(wx_messages::Column::LinkUrl)
            .column(wx_messages::Column::EventType)
            .column(wx_messages::Column::EventKey)
            .column(wx_messages::Column::ReplyMsgId)
            .column(wx_messages::Column::IsAutoReply)
            .column(wx_messages::Column::CreatedAt)
            .into_model::<WxMessagesResp>()
            .all(db)
            .await?;
        Ok(list)
    }
}
