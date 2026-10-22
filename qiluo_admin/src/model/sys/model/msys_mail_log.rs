pub use super::args::asys_mail_log::*;
pub use super::entity::sys_mail_log::{self, ActiveModel, Model as SysMailLogModel};
use crate::model::prelude::*;

impl SysMailLogModel {
    pub async fn list(arg: PageParams, search: MailLogSearch) -> Result<ListData<MailLogRes>> {
        let page_num = arg.page_num.unwrap_or(1);
        let page_per_size = arg.page_size.unwrap_or(10);
        let db = DB().await;
        let mut rmodel = sys_mail_log::Entity::find();

        if let Some(recipient) = &search.recipient {
            if !recipient.is_empty() {
                rmodel = rmodel.filter(sys_mail_log::Column::Recipient.contains(recipient));
            }
        }
        if let Some(subject) = &search.subject {
            if !subject.is_empty() {
                rmodel = rmodel.filter(sys_mail_log::Column::Subject.contains(subject));
            }
        }
        if let Some(status) = &search.status {
            if !status.is_empty() {
                rmodel = rmodel.filter(sys_mail_log::Column::Status.eq(status));
            }
        }
        if let Some(mail_type) = &search.mail_type {
            if !mail_type.is_empty() {
                rmodel = rmodel.filter(sys_mail_log::Column::MailType.eq(mail_type));
            }
        }

        let total = rmodel.clone().count(db).await?;
        let paginator = rmodel
            .order_by_desc(sys_mail_log::Column::CreatedAt)
            .into_model::<MailLogRes>()
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

    pub async fn add(
        recipient: String,
        subject: String,
        content_text: Option<String>,
        content_html: Option<String>,
        from_addr: Option<String>,
        reply_to: Option<String>,
        status: String,
        error_message: Option<String>,
        mail_type: Option<String>,
        created_by: Option<i64>,
    ) -> Result<i64> {
        let id = GID().await;
        let db = DB().await;
        let imodel = sys_mail_log::ActiveModel {
            id: Set(id),
            recipient: Set(recipient),
            subject: Set(subject),
            content_text: Set(content_text),
            content_html: Set(content_html),
            from_addr: Set(from_addr),
            reply_to: Set(reply_to),
            status: Set(status),
            error_message: Set(error_message),
            mail_type: Set(mail_type),
            created_by: Set(created_by),
            ..Default::default()
        };
        imodel.insert(db).await?;
        Ok(id)
    }

    pub async fn update_status(id: i64, status: &str, error_message: Option<String>) -> Result<()> {
        let db = DB().await;
        let model: Option<sys_mail_log::Model> = sys_mail_log::Entity::find_by_id(id).one(db).await?;
        if let Some(m) = model {
            let mut active: sys_mail_log::ActiveModel = m.into();
            active.status = Set(status.to_string());
            active.error_message = Set(error_message);
            active.update(db).await?;
        }
        Ok(())
    }
}
