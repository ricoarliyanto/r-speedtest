use serde::{Deserialize, Serialize};

/// 模板消息数据项 - 颜色值
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateDataItem {
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

/// 模板消息小程序跳转
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateMiniProgram {
    pub appid: String,
    pub pagepath: String,
}

/// 发送模板消息请求
#[derive(Debug, Clone, Serialize)]
pub struct SendTemplateMessageRequest {
    pub touser: String,
    pub template_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub miniprogram: Option<TemplateMiniProgram>,
    pub data: serde_json::Value,
}

/// 发送模板消息响应
#[derive(Debug, Clone, Deserialize)]
pub struct SendTemplateMessageResponse {
    pub errcode: i64,
    pub errmsg: String,
    pub msgid: i64,
}

/// 获取模板列表响应 - 单个模板
#[derive(Debug, Clone, Deserialize)]
pub struct TemplateItem {
    pub template_id: String,
    pub title: String,
    pub primary_industry: String,
    pub deputy_industry: String,
    pub content: String,
    pub example: String,
}

/// 获取模板列表响应
#[derive(Debug, Clone, Deserialize)]
pub struct GetTemplateListResponse {
    pub template_list: Option<Vec<TemplateItem>>,
}

/// 添加模板响应
#[derive(Debug, Clone, Deserialize)]
pub struct AddTemplateResponse {
    pub errcode: i64,
    pub errmsg: String,
    pub template_id: String,
}

/// 删除模板响应
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteTemplateResponse {
    pub errcode: i64,
    pub errmsg: String,
}

/// 获取模板行业响应
#[derive(Debug, Clone, Deserialize)]
pub struct IndustryItem {
    pub first_class: String,
    pub second_class: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetIndustryResponse {
    pub primary_industry: Option<IndustryItem>,
    pub secondary_industry: Option<IndustryItem>,
}

/// 设置模板行业请求
#[derive(Debug, Clone, Serialize)]
pub struct SetIndustryRequest {
    pub industry_id1: String,
    pub industry_id2: String,
}
