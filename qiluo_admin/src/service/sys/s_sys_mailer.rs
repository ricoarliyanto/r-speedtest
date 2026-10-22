use crate::model::prelude::*;
use crate::model::sys::model::msys_mail_log::{MailLogSearch, SysMailLogModel};
use crate::model::sys::model::msys_mail_template::{
    AddMailTemplateReq, EditMailTemplateReq, MailTemplateSearch, SysMailTemplateModel,
};
use crate::service::prelude::*;
use crate::worker::mailer::template::render_from_db;
use crate::worker::mailer::MailerWorker;
use crate::worker::mailer::Email;
use crate::worker::AppWorker;
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct SendEmailReq {
    /// 收件人邮箱
    #[validate(email(message = "邮箱格式不正确"))]
    pub to: String,
    /// 邮件主题
    #[validate(length(min = 1, message = "邮件主题不能为空"))]
    pub subject: String,
    /// 纯文本内容
    #[validate(length(min = 1, message = "邮件内容不能为空"))]
    pub text: String,
    /// HTML内容（可选）
    pub html: Option<String>,
    /// 发件人（可选，默认使用系统配置）
    pub from: Option<String>,
    /// 回复地址（可选，必须是邮箱格式）
    #[validate(email(message = "回复地址邮箱格式不正确"))]
    pub reply_to: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct SendTemplateEmailReq {
    /// 收件人邮箱
    #[validate(email(message = "邮箱格式不正确"))]
    pub to: String,
    /// 模板编码（数据库中的 code 字段）
    #[validate(length(min = 1, message = "模板编码不能为空"))]
    pub template_code: String,
    /// 发件人（可选）
    pub from: Option<String>,
    /// 回复地址（可选，必须是邮箱格式）
    #[validate(email(message = "回复地址邮箱格式不正确"))]
    pub reply_to: Option<String>,
    /// 模板变量
    pub locals: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct MailLogListReq {
    pub page_num: Option<u64>,
    pub page_size: Option<u64>,
    pub recipient: Option<String>,
    pub subject: Option<String>,
    pub status: Option<String>,
    pub mail_type: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct MailTemplateListReq {
    pub page_num: Option<u64>,
    pub page_size: Option<u64>,
    pub name: Option<String>,
    pub code: Option<String>,
    pub subject: Option<String>,
    pub status: Option<i32>,
}

/// 发送纯文本/HTML邮件
pub async fn send_email(uinfo: UserInfo, VJson(arg): VJson<SendEmailReq>) -> impl IntoResponse {
    // 先记录 pending 日志
    let log_id = match SysMailLogModel::add(
        arg.to.clone(),
        arg.subject.clone(),
        Some(arg.text.clone()),
        arg.html.clone(),
        arg.from.clone(),
        arg.reply_to.clone(),
        "pending".to_string(),
        None,
        Some("plain".to_string()),
        Some(uinfo.uid),
    )
    .await
    {
        Ok(id) => id,
        Err(e) => return ApiResponse::internal_server_error(format!("记录邮件日志失败: {}", e)),
    };

    let email = Email {
        log_id: Some(log_id),
        from: arg.from.clone(),
        to: arg.to.clone(),
        reply_to: arg.reply_to.clone(),
        subject: arg.subject.clone(),
        text: arg.text.clone(),
        html: arg.html.clone().unwrap_or_default(),
    };

    match MailerWorker::enqueue_sync(email).await {
        Ok(_) => ApiResponse::ok_with_msg(serde_json::json!({}), "邮件已加入发送队列"),
        Err(e) => {
            // 加入队列失败，更新日志为 failed
            let _ = SysMailLogModel::update_status(log_id, "failed", Some(format!("邮件发送失败: {}", e))).await;
            ApiResponse::internal_server_error(format!("邮件发送失败: {}", e))
        }
    }
}

/// 发送模板邮件（从数据库加载模板）
pub async fn send_template_email(
    uinfo: UserInfo,
    VJson(arg): VJson<SendTemplateEmailReq>,
) -> impl IntoResponse {
    match render_from_db(&arg.template_code, &arg.locals).await {
        Ok(content) => {
            // 先记录 pending 日志
            let log_id = match SysMailLogModel::add(
                arg.to.clone(),
                content.subject.clone(),
                Some(content.text.clone()),
                Some(content.html.clone()),
                arg.from.clone(),
                arg.reply_to.clone(),
                "pending".to_string(),
                None,
                Some("template".to_string()),
                Some(uinfo.uid),
            )
            .await
            {
                Ok(id) => id,
                Err(e) => return ApiResponse::internal_server_error(format!("记录邮件日志失败: {}", e)),
            };

            let email = Email {
                log_id: Some(log_id),
                from: arg.from.clone(),
                to: arg.to.clone(),
                reply_to: arg.reply_to.clone(),
                subject: content.subject,
                text: content.text,
                html: content.html,
            };

            match MailerWorker::enqueue_sync(email).await {
                Ok(_) => ApiResponse::ok_with_msg(serde_json::json!({}), "模板邮件已加入发送队列"),
                Err(e) => {
                    let _ = SysMailLogModel::update_status(log_id, "failed", Some(format!("模板邮件发送失败: {}", e))).await;
                    ApiResponse::internal_server_error(format!("模板邮件发送失败: {}", e))
                }
            }
        }
        Err(e) => ApiResponse::internal_server_error(format!("模板渲染失败: {}", e)),
    }
}

/// 获取邮件配置信息（脱敏）
pub async fn get_mailer_config() -> impl IntoResponse {
    let config = APPCOFIG.mailer.clone();
    match config {
        Some(mailer) => {
            let smtp_info = match &mailer.smtp {
                Some(smtp) => {
                    let auth_info = smtp.auth.as_ref().map(|a| {
                        let masked_pwd = if a.password.len() > 3 {
                            format!("{}****", &a.password[..3])
                        } else {
                            "****".to_string()
                        };
                        serde_json::json!({
                            "user": a.user,
                            "password": masked_pwd,
                        })
                    });
                    serde_json::json!({
                        "enable": smtp.enable,
                        "host": smtp.host,
                        "port": smtp.port,
                        "secure": smtp.secure,
                        "auth": auth_info,
                    })
                }
                None => serde_json::Value::Null,
            };
            ApiResponse::ok(serde_json::json!({
                "smtp": smtp_info,
                "stub": mailer.stub,
            }))
        }
        None => ApiResponse::ok(serde_json::json!({
            "smtp": null,
            "stub": false,
            "message": "邮件服务未配置",
        })),
    }
}

/// 获取邮件日志列表
pub async fn get_mail_log_list(VQuery(arg): VQuery<MailLogListReq>) -> impl IntoResponse {
    let page = PageParams {
        page_num: arg.page_num,
        page_size: arg.page_size,
    };
    let search = MailLogSearch {
        recipient: arg.recipient,
        subject: arg.subject,
        status: arg.status,
        mail_type: arg.mail_type,
    };
    match SysMailLogModel::list(page, search).await {
        Ok(data) => ApiResponse::ok(data),
        Err(e) => ApiResponse::internal_server_error(format!("获取邮件日志失败: {}", e)),
    }
}

// ====== 邮件模板 CRUD ======

/// 获取邮件模板列表
pub async fn get_mail_template_list(VQuery(arg): VQuery<MailTemplateListReq>) -> impl IntoResponse {
    let page = PageParams {
        page_num: arg.page_num,
        page_size: arg.page_size,
    };
    let search = MailTemplateSearch {
        name: arg.name,
        code: arg.code,
        subject: arg.subject,
        status: arg.status,
    };
    match SysMailTemplateModel::list(page, search).await {
        Ok(data) => ApiResponse::ok(data),
        Err(e) => ApiResponse::internal_server_error(format!("获取邮件模板列表失败: {}", e)),
    }
}

/// 新增邮件模板
pub async fn add_mail_template(VJson(arg): VJson<AddMailTemplateReq>) -> impl IntoResponse {
    if let Ok(Some(_)) = SysMailTemplateModel::find_by_code(&arg.code).await {
        return ApiResponse::internal_server_error(format!("模板编码 [{}] 已存在", arg.code));
    }
    match SysMailTemplateModel::add(arg).await {
        Ok(id) => ApiResponse::ok_with_msg(serde_json::json!({ "id": id }), "新增成功"),
        Err(e) => ApiResponse::internal_server_error(format!("新增邮件模板失败: {}", e)),
    }
}

/// 编辑邮件模板
pub async fn edit_mail_template(VJson(arg): VJson<EditMailTemplateReq>) -> impl IntoResponse {
    if let Some(ref code) = arg.code {
        if let Ok(Some(existing)) = SysMailTemplateModel::find_by_code(code).await {
            if existing.id != arg.id {
                return ApiResponse::internal_server_error(format!(
                    "模板编码 [{}] 已被其他模板使用",
                    code
                ));
            }
        }
    }
    match SysMailTemplateModel::edit(arg).await {
        Ok(_) => ApiResponse::ok_with_msg(serde_json::json!({}), "编辑成功"),
        Err(e) => ApiResponse::internal_server_error(format!("编辑邮件模板失败: {}", e)),
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct DeleteMailTemplateReq {
    pub id: i64,
}

/// 删除邮件模板
pub async fn delete_mail_template(VQuery(arg): VQuery<DeleteMailTemplateReq>) -> impl IntoResponse {
    match SysMailTemplateModel::delete_by_id(arg.id).await {
        Ok(_) => ApiResponse::ok_with_msg(serde_json::json!({}), "删除成功"),
        Err(e) => ApiResponse::internal_server_error(format!("删除邮件模板失败: {}", e)),
    }
}
