pub use super::args::atest_article::*;
pub use super::entity::test_article::{self, ActiveModel, Model as TestArticleModel};
use crate::model::prelude::*;

impl TestArticleModel {
    pub async fn list(arg: PageParams, search: TestArticleSearch) -> Result<ListData<TestArticleResp>> {
        let page_num = arg.page_num.unwrap_or(1);
        let page_per_size = arg.page_size.unwrap_or(10);
        let db = DB().await;
        let mut rmodel = test_article::Entity::find();
        if let Some(title) = search.title {
            rmodel = rmodel.filter(test_article::Column::Title.contains(title));
        }
        if let Some(content) = search.content {
            rmodel = rmodel.filter(test_article::Column::Content.contains(content));
        }
        if let Some(author) = search.author {
            rmodel = rmodel.filter(test_article::Column::Author.contains(author));
        }
        if let Some(password) = search.password {
            rmodel = rmodel.filter(test_article::Column::Password.contains(password));
        }
        if let Some(cover) = search.cover {
            rmodel = rmodel.filter(test_article::Column::Cover.contains(cover));
        }
        let total = rmodel.clone().count(db).await?;
        // 排序
        rmodel = rmodel.order_by_desc(test_article::Column::UpdatedAt);
        let paginator = rmodel
            .into_model::<TestArticleResp>()
            .paginate(db, page_per_size);
        let total_pages = paginator.num_pages().await?;
        let list = paginator.fetch_page(page_num - 1).await?;
        Ok(ListData { list, total, total_pages, page_num })
    }

    pub async fn add(arg: TestArticleAdd) -> Result<String> {
        let db = DB().await;
        let id = GID().await;
        let now = Local::now().naive_local();
        let model = test_article::ActiveModel {
            id: Set(id),
            category_id: Set(arg.category_id),
            title: Set(arg.title),
            content: Set(arg.content),
            author: Set(arg.author),
            password: Set(arg.password),
            is_published: Set(arg.is_published),
            view_count: Set(arg.view_count),
            download_count: Set(arg.download_count),
            rating: Set(arg.rating),
            cover: Set(arg.cover),
            ..Default::default()
        };
        test_article::Entity::insert(model).exec(db).await?;
        Ok(format!("Successfully added record with id: {}", id))
    }

    pub async fn edit(arg: TestArticleEdit) -> Result<String> {
        let db = DB().await;
        let model = test_article::Entity::find_by_id(arg.id).one(db).await?;
        if let Some(model) = model {
            let mut active_model: test_article::ActiveModel = model.into();
            active_model.category_id = Set(arg.category_id);
            active_model.title = Set(arg.title);
            active_model.content = Set(arg.content);
            active_model.author = Set(arg.author);
            active_model.password = Set(arg.password);
            active_model.is_published = Set(arg.is_published);
            active_model.view_count = Set(arg.view_count);
            active_model.download_count = Set(arg.download_count);
            active_model.rating = Set(arg.rating);
            active_model.cover = Set(arg.cover);
            active_model.updated_at = Set(Some(Local::now().naive_local()));
            let _ = active_model.update(db).await?;
            Ok("Successfully updated record".to_string())
        } else {
            Err("Record not found".into())
        }
    }

    pub async fn del(arg: TestArticleDel) -> Result<String> {
        let db = DB().await;
        let result = test_article::Entity::delete_by_id(arg.id).exec(db).await?;
        if result.rows_affected > 0 {
            Ok("Success".to_string())
        } else {
            Err("delete failed".into())
        }
    }
}