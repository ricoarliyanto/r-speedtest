use serde::{Deserialize, Serialize};
use crate::worker::common::{Worker,WorkerOpts};
mod email_sender;
pub mod template;
use crate::config::APPCOFIG;
use crate::worker::AppWorker;
use crate::common::error::Result;
use async_trait::async_trait;
pub use email_sender::EmailSender;
use tokio::sync::OnceCell;
use crate::model::sys::model::msys_mail_log::SysMailLogModel;

static EMAILSENDER: OnceCell<EmailSender> = OnceCell::const_new();
use self::template::Template;

async fn email_init() -> EmailSender {
    let mailer_config = APPCOFIG.mailer.clone().unwrap();
    if mailer_config.stub {
        let smtp_config = mailer_config.smtp.as_ref();
        EmailSender::stub_with_from(
            smtp_config.and_then(|c| c.from.clone()),
            smtp_config.and_then(|c| c.auth.as_ref().map(|a| a.user.clone())),
        )
    } else {
        let smtp_config = mailer_config.smtp.unwrap();
        EmailSender::smtp(&smtp_config).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Email {
    /// Mail log ID for status tracking
    pub log_id: Option<i64>,
    /// Mailbox to `From` header
    pub from: Option<String>,
    /// Mailbox to `To` header
    pub to: String,
    /// Mailbox to `ReplyTo` header
    pub reply_to: Option<String>,
    /// Subject header to message
    pub subject: String,
    /// Plain text message
    pub text: String,
    /// HTML template
    pub html: String,
}
#[derive(Clone)]
pub struct MailerWorker {}

impl AppWorker<Email> for MailerWorker {
    fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Worker<Email> for MailerWorker {
    /// Returns options for the mailer worker, specifying the queue to process.
    fn opts() -> WorkerOpts<Email, Self> { 
        WorkerOpts::new().queue("mailer")
    }

    /// Performs the email sending operation using the provided [`AppContext`]
    /// and email details.
    async fn perform(&self, email: Email) ->  Result<()> {
        let mailer = EMAILSENDER.get_or_init(email_init).await;
        let result = mailer.mail(&email).await;

        // Update mail log status based on send result
        if let Some(log_id) = email.log_id {
            match &result {
                Ok(_) => {
                    let _ = SysMailLogModel::update_status(log_id, "success", None).await;
                }
                Err(e) => {
                    let _ = SysMailLogModel::update_status(log_id, "failed", Some(e.to_string())).await;
                }
            }
        }

        result?;
        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct Args {
    pub from: Option<String>,
    pub to: String,
    pub reply_to: Option<String>,
    pub locals: serde_json::Value,
}

pub async fn mail_template(dir: String, args: Args) -> Result<()> {
    let content = Template::new(dir).render(&args.locals)?;
    mail(&Email {
        log_id: None,
        from: args.from.clone(),
        to: args.to.clone(),
        reply_to: args.reply_to.clone(),
        subject: content.subject,
        text: content.text,
        html: content.html,
    })
    .await 
}

async fn mail(email: &Email) -> Result<()> {
    MailerWorker::enqueue_sync(email.clone())
        .await
        .map_err(Box::from)?;
    Ok(())
}
