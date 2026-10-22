use super::atemplate::*;
use serde_json::Value;

/// 模板消息构建器
pub struct TemplateMessageBuilder {
    touser: String,
    template_id: String,
    url: Option<String>,
    miniprogram: Option<TemplateMiniProgram>,
    data: serde_json::Map<String, Value>,
}

impl TemplateMessageBuilder {
    pub fn new(touser: &str, template_id: &str) -> Self {
        Self {
            touser: touser.to_string(),
            template_id: template_id.to_string(),
            url: None,
            miniprogram: None,
            data: serde_json::Map::new(),
        }
    }

    /// 设置跳转URL
    pub fn url(mut self, url: &str) -> Self {
        self.url = Some(url.to_string());
        self
    }

    /// 设置小程序跳转
    pub fn miniprogram(mut self, appid: &str, pagepath: &str) -> Self {
        self.miniprogram = Some(TemplateMiniProgram {
            appid: appid.to_string(),
            pagepath: pagepath.to_string(),
        });
        self
    }

    /// 添加模板数据项 (key, value, color可选)
    pub fn data_item(mut self, key: &str, value: &str, color: Option<&str>) -> Self {
        let mut item = serde_json::Map::new();
        item.insert("value".to_string(), Value::String(value.to_string()));
        if let Some(c) = color {
            item.insert("color".to_string(), Value::String(c.to_string()));
        }
        self.data.insert(key.to_string(), Value::Object(item));
        self
    }

    /// 添加 first 字段
    pub fn first(self, value: &str, color: Option<&str>) -> Self {
        self.data_item("first", value, color)
    }

    /// 添加 remark 字段
    pub fn remark(self, value: &str, color: Option<&str>) -> Self {
        self.data_item("remark", value, color)
    }

    /// 添加关键字数据项 (keyword1, keyword2, ...)
    pub fn keyword(self, index: u32, value: &str, color: Option<&str>) -> Self {
        self.data_item(&format!("keyword{}", index), value, color)
    }

    /// 构建发送请求
    pub fn build(self) -> SendTemplateMessageRequest {
        SendTemplateMessageRequest {
            touser: self.touser,
            template_id: self.template_id,
            url: self.url,
            miniprogram: self.miniprogram,
            data: Value::Object(self.data),
        }
    }
}
