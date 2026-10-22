use crate::common::error::Result;
use crate::common::tera;
use crate::model::sys::model::msys_mail_template::SysMailTemplateModel;
use fs_err as fs;
use std::env;

/// The filename for the subject template file.
const SUBJECT: &str = "subject.t";
/// The filename for the HTML template file.
const HTML: &str = "html.t";
/// The filename for the plain text template file.
const TEXT: &str = "text.t";

fn embedded_file(dir: String, name: &str) -> Result<String> {
    let path = env::current_dir().unwrap();
    let files = path.join(dir).join(name);
    let content = fs::read_to_string(files).expect("msg");
    Ok(content)
}

#[derive(Clone, Debug)]
pub struct Content {
    pub subject: String,
    pub text: String,
    pub html: String,
}

#[derive(Debug, Clone)]
pub struct Template {
    dir: String,
}

impl Template {
    pub const fn new(dir: String) -> Self {
        Self { dir }
    }

    /// 从文件系统渲染模板
    pub fn render(&self, locals: &serde_json::Value) -> Result<Content> {
        let subject_t = embedded_file(self.dir.clone(), SUBJECT)?;
        let text_t = embedded_file(self.dir.clone(), TEXT)?;
        let html_t = embedded_file(self.dir.clone(), HTML)?;

        let text = tera::render_string(&text_t, locals);

        let text = text.unwrap();
        let html = tera::render_string(&html_t, locals)?;
        let subject = tera::render_string(&subject_t, locals)?;

        Ok(Content {
            subject,
            text,
            html,
        })
    }
}

/// 从数据库渲染模板（通过模板编码）
pub async fn render_from_db(code: &str, locals: &serde_json::Value) -> Result<Content> {
    let tpl = SysMailTemplateModel::find_by_code(code).await?;
    match tpl {
        Some(tpl) => {
            if tpl.status != 1 {
                return Err(crate::common::error::Error::Message(format!(
                    "邮件模板 [{}] 已禁用",
                    code
                )));
            }
            let subject = tera::render_string(&tpl.subject, locals)?;
            let text = match &tpl.text_content {
                Some(t) => tera::render_string(t, locals).unwrap_or_default(),
                None => String::new(),
            };
            let html = match &tpl.html_content {
                Some(t) => tera::render_string(t, locals).unwrap_or_default(),
                None => String::new(),
            };
            Ok(Content {
                subject,
                text,
                html,
            })
        }
        None => Err(crate::common::error::Error::Message(format!(
            "邮件模板 [{}] 不存在",
            code
        ))),
    }
}
