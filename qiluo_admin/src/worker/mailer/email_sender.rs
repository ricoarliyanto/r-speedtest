use super::Email;
use crate::common::error::{Error, Result};
use lettre::{
    AsyncTransport, Message, Tokio1Executor, Transport,
    message::{Mailbox, MultiPart, SinglePart},
    transport::smtp::authentication::Credentials,
};
use tracing::info;

#[derive(Clone)]
pub enum EmailTransport {
    /// SMTP (Simple Mail Transfer Protocol) transport.
    Smtp(lettre::AsyncSmtpTransport<lettre::Tokio1Executor>),
    /// Test/stub transport for testing purposes.
    Test(lettre::transport::stub::StubTransport),
}

#[derive(Clone)]
pub struct EmailSender {
    pub transport: EmailTransport,
    /// Default from sender from config
    pub default_from: Option<String>,
    /// Authenticated SMTP user (the actual sender address)
    pub auth_user: Option<String>,
}

impl EmailSender {
    pub fn stub() -> Self {
        Self {
            transport: EmailTransport::Test(lettre::transport::stub::StubTransport::new_ok()),
            default_from: None,
            auth_user: None,
        }
    }

    pub fn stub_with_from(default_from: Option<String>, auth_user: Option<String>) -> Self {
        Self {
            transport: EmailTransport::Test(lettre::transport::stub::StubTransport::new_ok()),
            default_from,
            auth_user,
        }
    }

    pub fn smtp(config: &crate::config::appconfig::SmtpMailer) -> Result<Self> {
        let mut email_builder = if config.secure {
            lettre::AsyncSmtpTransport::<Tokio1Executor>::relay(&config.host)
                .map_err(|error| {
                    tracing::error!(err.msg = %error, err.detail = ?error, "smtp_init_error");
                    Error::Message("error initialize smtp mailer".to_string())
                })?
                .port(config.port)
        } else {
            lettre::AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&config.host)
                .port(config.port)
        };

        if let Some(auth) = config.auth.as_ref() {
            email_builder = email_builder
                .credentials(Credentials::new(auth.user.clone(), auth.password.clone()));
        }

        Ok(Self {
            transport: EmailTransport::Smtp(email_builder.build()),
            default_from: config.from.clone(),
            auth_user: config.auth.as_ref().map(|a| a.user.clone()),
        })
    }

    pub async fn mail(&self, email: &Email) -> Result<String> {
        let content = if email.html.trim().is_empty() {
            // 只有纯文本
            MultiPart::alternative().singlepart(SinglePart::plain(email.text.clone()))
        } else {
            // 同时提供纯文本 + HTML
            MultiPart::alternative_plain_html(email.text.clone(), email.html.clone())
        };

        let from_input = email
            .from
            .clone()
            .or_else(|| self.default_from.clone())
            .ok_or_else(|| Error::Message("no from sender configured".to_string()))?;

        // Try to parse as full "Name <email>" format first.
        // If parsing fails, treat it as a display name only and pair with the auth user email.
        let from_mailbox = match from_input.parse::<Mailbox>() {
            Ok(mb) => mb,
            Err(_) => {
                // User typed a display name like "祺洛科技", not a full address
                let auth_email = self.auth_user.as_deref().ok_or_else(|| {
                    Error::Message("发件人仅填了显示名，但未配置SMTP认证邮箱".to_string())
                })?;
                Mailbox::new(
                    Some(from_input.parse().unwrap()),
                    auth_email.parse().unwrap(),
                )
            }
        };

        // Force the email address to match the authenticated SMTP user,
        // because most SMTP servers (163, QQ, Gmail) reject sending from a different address.
        let from_mailbox = if let Some(ref auth_email) = self.auth_user {
            if from_mailbox.email.to_string() != *auth_email {
                Mailbox::new(from_mailbox.name.clone(), auth_email.parse().unwrap())
            } else {
                from_mailbox
            }
        } else {
            from_mailbox
        };

        let mut builder = Message::builder()
            .from(from_mailbox)
            .to(email.to.parse()?);

        if let Some(reply_to) = &email.reply_to {
            builder = builder.reply_to(reply_to.parse()?);
        }

        let msg = builder
            .subject(email.subject.clone())
            .multipart(content)
            .map_err(|error| {
                tracing::error!(err.msg = %error, err.detail = ?error, "email_building_error");
                Error::Message("error building email message".to_owned())
            })?;

        match &self.transport {
            EmailTransport::Smtp(xp) => {
                match xp.send(msg).await {
                    Ok(_) => {
                        info!("Email sent successfully!");
                        Ok("sc".to_owned())
                    }
                    Err(e) => {
                        tracing::error!(err.msg = %e, err.detail = ?e, "smtp_send_error");
                        Err(Error::Message(format!("邮件发送失败: {}", e)))
                    }
                }
            }
            EmailTransport::Test(xp) => {
                xp.send(&msg).map_err(|e| {
                    tracing::error!(err.msg = %e, err.detail = ?e, "test_send_error");
                    Error::Message(format!("邮件发送失败: {}", e))
                })?;
                Ok("sc".to_owned())
            }
        }
    }
}
