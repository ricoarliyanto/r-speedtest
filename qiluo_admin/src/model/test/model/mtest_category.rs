pub use super::args::atest_category::*;
pub use super::entity::test_category::{self, ActiveModel, Model as TestCategoryModel};
use crate::model::prelude::*;

impl TestCategoryModel {
    pub async fn list(arg: PageParams, search: TestCategorySearch) -> Result<ListData<TestCategoryResp>> {
        let page_num = arg.page_num.unwrap_or(1);
        let page_per_size = arg.page_size.unwrap_or(10);
        let db = DB().await;
        let mut rmodel = test_category::Entity::find();
        if let Some(name) = search.name {
            rmodel = rmodel.filter(test_category::Column::Name.contains(name));
        }
        if let Some(description) = search.description {
            rmodel = rmodel.filter(test_category::Column::Description.contains(description));
        }
        if let Some(status) = search.status {
            rmodel = rmodel.filter(test_category::Column::Status.contains(status));
        }
        if let Some(cover) = search.cover {
            rmodel = rmodel.filter(test_category::Column::Cover.contains(cover));
        }
        let total = rmodel.clone().count(db).await?;
        // 排序
        rmodel = rmodel.order_by_desc(test_category::Column::Weight);
	rmodel = rmodel.order_by_asc(test_category::Column::Sort);
        let paginator = rmodel
            .into_model::<TestCategoryResp>()
            .paginate(db, page_per_size);
        let total_pages = paginator.num_pages().await?;
        let list = paginator.fetch_page(page_num - 1).await?;
        Ok(ListData { list, total, total_pages, page_num })
    }

    pub async fn add(arg: TestCategoryAdd) -> Result<String> {
        let db = DB().await;
        let id = GID().await;
        let now = Local::now().naive_local();
        let model = test_category::ActiveModel {
            id: Set(id),
            name: Set(arg.name),
            description: Set(arg.description),
            sort: Set(arg.sort),
            status: Set(arg.status),
            cover: Set(arg.cover),
            is_active: Set(arg.is_active),
            weight: Set(arg.weight),
            view_count: Set(arg.view_count),
            ..Default::default()
        };
        test_category::Entity::insert(model).exec(db).await?;
        Ok(format!("Successfully added record with id: {}", id))
    }

    pub async fn edit(arg: TestCategoryEdit) -> Result<String> {
        let db = DB().await;
        let model = test_category::Entity::find_by_id(arg.id).one(db).await?;
        if let Some(model) = model {
            let mut active_model: test_category::ActiveModel = model.into();
            active_model.name = Set(arg.name);
            active_model.description = Set(arg.description);
            active_model.sort = Set(arg.sort);
            active_model.status = Set(arg.status);
            active_model.cover = Set(arg.cover);
            active_model.is_active = Set(arg.is_active);
            active_model.weight = Set(arg.weight);
            active_model.view_count = Set(arg.view_count);
            active_model.updated_at = Set(Some(Local::now().naive_local()));
            let _ = active_model.update(db).await?;
            Ok("Successfully updated record".to_string())
        } else {
            Err("Record not found".into())
        }
    }

    pub async fn del(arg: TestCategoryDel) -> Result<String> {
        let db = DB().await;
        let result = test_category::Entity::delete_by_id(arg.id).exec(db).await?;
        if result.rows_affected > 0 {
            Ok("Success".to_string())
        } else {
            Err("delete failed".into())
        }
    }
}