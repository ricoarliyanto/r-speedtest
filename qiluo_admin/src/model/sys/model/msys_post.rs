use crate::model::prelude::*;
pub use super::entity::sys_post::{self, ActiveModel, Model as SysPostModel};


impl SysPostModel{
    pub async fn list(arg: PageParams, search: crate::model::sys::args::asys_post::SysPostSearch) -> Result<ListData<crate::model::sys::args::asys_post::SysPostResp>> {
        let db = DB().await;
        let rmodel = sys_post::Entity::find();
        let total = rmodel.clone().count(db).await?;
        let paginator = rmodel
            .into_model::<crate::model::sys::args::asys_post::SysPostResp>()
            .paginate(db, 10);
        let total_pages = paginator.num_pages().await?;
        let list = paginator.fetch_page(0).await?;
        Ok(ListData { list, total, total_pages, page_num: 1 })
    }
    pub async fn add(_arg: crate::model::sys::args::asys_post::SysPostAdd) -> Result<String> {
        Ok("Success".to_string())
    }
    pub async fn del(_arg: crate::model::sys::args::asys_post::SysPostDel) -> Result<String> {
        Ok("Success".to_string())
    }
    pub async fn edit(_arg: crate::model::sys::args::asys_post::SysPostEdit) -> Result<String> {
        Ok("Success".to_string())
    }
}