pub use super::args::awx_materials::*;
pub use super::entity::wx_materials::{self, ActiveModel, Model as WxMaterialsModel};
use crate::model::prelude::*;

impl WxMaterialsModel {
    pub async fn find_by_id(id: i64) -> Result<Option<WxMaterialsModel>> {
        let db = DB().await;
        let rmodel = wx_materials::Entity::find_by_id(id).one(db).await?;
        Ok(rmodel)
    }

    pub async fn list(
        arg: PageParams,
        search: WxMaterialsSearch,
    ) -> Result<ListData<WxMaterialsResp>> {
        let page_num = arg.page_num.unwrap_or(1);
        let page_per_size = arg.page_size.unwrap_or(10);
        let db = DB().await;
        let mut rmodel = wx_materials::Entity::find();
        if let Some(account_id) = search.account_id {
            rmodel = rmodel.filter(wx_materials::Column::AccountId.eq(account_id));
        }
        if let Some(media_type) = search.media_type {
            rmodel = rmodel.filter(wx_materials::Column::MediaType.eq(media_type));
        }
        if let Some(name) = search.name {
            rmodel = rmodel.filter(wx_materials::Column::Name.contains(&name));
        }
        if let Some(is_permanent) = search.is_permanent {
            rmodel = rmodel.filter(wx_materials::Column::IsPermanent.eq(is_permanent));
        }
        if let Some(sync_status) = search.sync_status {
            rmodel = rmodel.filter(wx_materials::Column::SyncStatus.eq(sync_status));
        }
        let total = rmodel.clone().count(db).await?;
        let paginator = rmodel
            .select_only()
            .column(wx_materials::Column::Id)
            .column(wx_materials::Column::AccountId)
            .column(wx_materials::Column::MediaType)
            .column(wx_materials::Column::MediaId)
            .column(wx_materials::Column::Name)
            .column(wx_materials::Column::Url)
            .column(wx_materials::Column::LocalPath)
            .column(wx_materials::Column::FileSize)
            .column(wx_materials::Column::ContentType)
            .column(wx_materials::Column::Width)
            .column(wx_materials::Column::Height)
            .column(wx_materials::Column::Duration)
            .column(wx_materials::Column::Description)
            .column(wx_materials::Column::Title)
            .column(wx_materials::Column::Introduction)
            .column(wx_materials::Column::ThumbMediaId)
            .column(wx_materials::Column::ThumbUrl)
            .column(wx_materials::Column::ContentSourceUrl)
            .column(wx_materials::Column::Digest)
            .column(wx_materials::Column::Author)
            .column(wx_materials::Column::Content)
            .column(wx_materials::Column::NewsItems)
            .column(wx_materials::Column::IsPermanent)
            .column(wx_materials::Column::SyncStatus)
            .column(wx_materials::Column::SyncedAt)
            .column(wx_materials::Column::CreatedAt)
            .column(wx_materials::Column::UpdatedAt)
            .order_by_desc(wx_materials::Column::CreatedAt)
            .into_model::<WxMaterialsResp>()
            .paginate(db, page_per_size);
        let total_pages = paginator.num_pages().await?;
        let list = paginator.fetch_page(page_num - 1).await?;
        Ok(ListData { list, total, total_pages, page_num })
    }

    pub async fn add(arg: WxMaterialsAdd) -> Result<String> {
        let db = DB().await;
        let id = GID().await;
        let model = wx_materials::ActiveModel {
            id: Set(id),
            account_id: Set(arg.account_id),
            media_type: Set(arg.media_type),
            media_id: Set(arg.media_id),
            name: Set(arg.name),
            url: Set(arg.url),
            local_path: Set(arg.local_path),
            file_size: Set(arg.file_size),
            content_type: Set(arg.content_type),
            width: Set(arg.width),
            height: Set(arg.height),
            duration: Set(arg.duration),
            description: Set(arg.description),
            title: Set(arg.title),
            introduction: Set(arg.introduction),
            thumb_media_id: Set(arg.thumb_media_id),
            thumb_url: Set(arg.thumb_url),
            content_source_url: Set(arg.content_source_url),
            digest: Set(arg.digest),
            author: Set(arg.author),
            content: Set(arg.content),
            news_items: Set(arg.news_items),
            is_permanent: Set(arg.is_permanent.unwrap_or(0)),
            sync_status: Set(arg.sync_status.unwrap_or(0)),
            synced_at: Set(arg.synced_at),
            created_at: Set(arg.created_at),
            updated_at: Set(arg.updated_at)
        };
        let result = wx_materials::Entity::insert(model).exec(db).await?;
        Ok(format!("Successfully added record with id: {}", result.last_insert_id))
    }

    pub async fn edit(arg: WxMaterialsEdit) -> Result<String> {
        let db = DB().await;
        let model = wx_materials::Entity::find_by_id(arg.id).one(db).await?;
        if let Some(model) = model {
            let mut active_model: wx_materials::ActiveModel = model.into();
            if let Some(media_type) = arg.media_type {
                active_model.media_type = Set(media_type);
            }
            if let Some(media_id) = arg.media_id {
                active_model.media_id = Set(Some(media_id));
            }
            active_model.name = Set(arg.name);
            active_model.url = Set(arg.url);
            active_model.local_path = Set(arg.local_path);
            active_model.file_size = Set(arg.file_size);
            active_model.content_type = Set(arg.content_type);
            active_model.width = Set(arg.width);
            active_model.height = Set(arg.height);
            active_model.duration = Set(arg.duration);
            active_model.description = Set(arg.description);
            active_model.title = Set(arg.title);
            active_model.introduction = Set(arg.introduction);
            active_model.thumb_media_id = Set(arg.thumb_media_id);
            active_model.thumb_url = Set(arg.thumb_url);
            active_model.content_source_url = Set(arg.content_source_url);
            active_model.digest = Set(arg.digest);
            active_model.author = Set(arg.author);
            active_model.content = Set(arg.content);
            active_model.news_items = Set(arg.news_items);
            if let Some(is_permanent) = arg.is_permanent {
                active_model.is_permanent = Set(is_permanent);
            }
            if let Some(sync_status) = arg.sync_status {
                active_model.sync_status = Set(sync_status);
            }
            active_model.synced_at = Set(arg.synced_at);
            active_model.updated_at = Set(arg.updated_at);
            let _ = active_model.update(db).await?;
            Ok("Successfully updated record".to_string())
        } else {
            Err("Record not found".into())
        }
    }

    pub async fn del(arg: WxMaterialsDel) -> Result<String> {
        let db = DB().await;
        let result = wx_materials::Entity::delete_by_id(arg.id).exec(db).await?;
        if result.rows_affected > 0 {
            Ok("Success".to_string())
        } else {
            Err("delete failed".into())
        }
    }

    /// 根据account_id和media_id查找素材
    pub async fn find_by_media_id(account_id: i64, media_id: &str) -> Result<Option<WxMaterialsModel>> {
        let db = DB().await;
        let rmodel = wx_materials::Entity::find()
            .filter(wx_materials::Column::AccountId.eq(account_id))
            .filter(wx_materials::Column::MediaId.eq(media_id))
            .one(db)
            .await?;
        Ok(rmodel)
    }

    /// 删除指定公众号的所有素材记录
    pub async fn delete_by_account(account_id: i64) -> Result<String> {
        let db = DB().await;
        let result = wx_materials::Entity::delete_many()
            .filter(wx_materials::Column::AccountId.eq(account_id))
            .exec(db)
            .await?;
        Ok(format!("Deleted {} records", result.rows_affected))
    }

    /// 批量插入同步的素材
    pub async fn batch_add(items: Vec<WxMaterialsAdd>) -> Result<String> {
        let db = DB().await;
        let mut models = Vec::new();
        for arg in items {
            let id = GID().await;
            models.push(wx_materials::ActiveModel {
                id: Set(id),
                account_id: Set(arg.account_id),
                media_type: Set(arg.media_type),
                media_id: Set(arg.media_id),
                name: Set(arg.name),
                url: Set(arg.url),
                local_path: Set(arg.local_path),
                file_size: Set(arg.file_size),
                content_type: Set(arg.content_type),
                width: Set(arg.width),
                height: Set(arg.height),
                duration: Set(arg.duration),
                description: Set(arg.description),
                title: Set(arg.title),
                introduction: Set(arg.introduction),
                thumb_media_id: Set(arg.thumb_media_id),
                thumb_url: Set(arg.thumb_url),
                content_source_url: Set(arg.content_source_url),
                digest: Set(arg.digest),
                author: Set(arg.author),
                content: Set(arg.content),
                news_items: Set(arg.news_items),
                is_permanent: Set(arg.is_permanent.unwrap_or(0)),
                sync_status: Set(arg.sync_status.unwrap_or(1)),
                synced_at: Set(arg.synced_at),
                created_at: Set(arg.created_at),
                updated_at: Set(arg.updated_at)
            });
        }
        let result = wx_materials::Entity::insert_many(models).exec(db).await?;
        Ok(format!("Batch inserted {} records", result.last_insert_id))
    }
}
