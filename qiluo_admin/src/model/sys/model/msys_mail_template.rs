pub use super::args::asys_mail_template::*;
pub use super::entity::sys_mail_template::{self, ActiveModel, Model as SysMailTemplateModel};
use crate::model::prelude::*;

impl SysMailTemplateModel {
    pub async fn list(arg: PageParams, search: MailTemplateSearch) -> Result<ListData<MailTemplateRes>> {
        let page_num = arg.page_num.unwrap_or(1);
        let page_per_size = arg.page_size.unwrap_or(10);
        let db = DB().await;
        let mut rmodel = sys_mail_template::Entity::find();

        if let Some(name) = &search.name {
            if !name.is_empty() {
                rmodel = rmodel.filter(sys_mail_template::Column::Name.contains(name));
            }
        }
        if let Some(code) = &search.code {
            if !code.is_empty() {
                rmodel = rmodel.filter(sys_mail_template::Column::Code.contains(code));
            }
        }
        if let Some(subject) = &search.subject {
            if !subject.is_empty() {
                rmodel = rmodel.filter(sys_mail_template::Column::Subject.contains(subject));
            }
        }
        if let Some(status) = search.status {
            rmodel = rmodel.filter(sys_mail_template::Column::Status.eq(status));
        }

        let total = rmodel.clone().count(db).await?;
        let paginator = rmodel
            .order_by_desc(sys_mail_template::Column::CreatedAt)
            .into_model::<MailTemplateRes>()
            .paginate(db, page_per_size);
        let total_pages = paginator.num_pages().await?;
        let list = paginator.fetch_page(page_num - 1).await?;
        let res = ListData {
            list,
            total,
            total_pages,
            page_num,
        };
        Ok(res)
    }

    pub async fn add(arg: AddMailTemplateReq) -> Result<i64> {
        let id = GID().await;
        let db = DB().await;
        let imodel = sys_mail_template::ActiveModel {
            id: Set(id),
            name: Set(arg.name),
            code: Set(arg.code),
            subject: Set(arg.subject),
            text_content: Set(arg.text_content),
            html_content: Set(arg.html_content),
            description: Set(arg.description),
            status: Set(arg.status.unwrap_or(1)),
            ..Default::default()
        };
        imodel.insert(db).await?;
        Ok(id)
    }

    pub async fn edit(arg: EditMailTemplateReq) -> Result<()> {
        let db = DB().await;
        let mut umodel = sys_mail_template::ActiveModel {
            id: Set(arg.id),
            ..Default::default()
        };
        if let Some(name) = arg.name {
            umodel.name = Set(name);
        }
        if let Some(code) = arg.code {
            umodel.code = Set(code);
        }
        if let Some(subject) = arg.subject {
            umodel.subject = Set(subject);
        }
        if let Some(text_content) = arg.text_content {
            umodel.text_content = Set(Some(text_content));
        }
        if let Some(html_content) = arg.html_content {
            umodel.html_content = Set(Some(html_content));
        }
        if let Some(description) = arg.description {
            umodel.description = Set(Some(description));
        }
        if let Some(status) = arg.status {
            umodel.status = Set(status);
        }
        umodel.update(db).await?;
        Ok(())
    }

    pub async fn delete_by_id(id: i64) -> Result<()> {
        let db = DB().await;
        sys_mail_template::Entity::delete_by_id(id)
            .exec(db)
            .await?;
        Ok(())
    }

    pub async fn find_by_code(code: &str) -> Result<Option<MailTemplateRes>> {
        let db = DB().await;
        let result = sys_mail_template::Entity::find()
            .filter(sys_mail_template::Column::Code.eq(code))
            .into_model::<MailTemplateRes>()
            .one(db)
            .await?;
        Ok(result)
    }

    pub async fn find_by_id(id: i64) -> Result<Option<MailTemplateRes>> {
        let db = DB().await;
        let result = sys_mail_template::Entity::find()
            .filter(sys_mail_template::Column::Id.eq(id))
            .into_model::<MailTemplateRes>()
            .one(db)
            .await?;
        Ok(result)
    }
}
