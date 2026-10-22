use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Debug)]
pub struct WechatRequest {
    pub signature: String,
    pub timestamp: String,
    pub nonce: String,
    pub echostr: Option<String>,
    pub openid: Option<String>,
    pub xml: Option<String>,
}

#[derive(Deserialize, Debug, Validate)]
pub struct WeChatVerifyQuery {
    pub signature: String,
    pub timestamp: String,
    pub nonce: String,
    pub echostr: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WechatMessage {
    #[serde(rename = "ToUserName")]
    pub to_user_name: String,

    #[serde(rename = "FromUserName")]
    pub from_user_name: String,

    #[serde(rename = "CreateTime")]
    pub create_time: i64,

    #[serde(rename = "MsgType")]
    pub msg_type: String,

    #[serde(rename = "Content")]
    pub content: Option<String>,

    #[serde(rename = "MsgId")]
    pub msg_id: Option<i64>,

    #[serde(rename = "PicUrl")]
    pub pic_url: Option<String>,

    #[serde(rename = "MediaId")]
    pub media_id: Option<String>,

    #[serde(rename = "Format")]
    pub format: Option<String>,

    #[serde(rename = "Recognition")]
    pub recognition: Option<String>,

    #[serde(rename = "ThumbMediaId")]
    pub thumb_media_id: Option<String>,

    #[serde(rename = "Location_X")]
    pub location_x: Option<f64>,

    #[serde(rename = "Location_Y")]
    pub location_y: Option<f64>,

    #[serde(rename = "Scale")]
    pub scale: Option<i32>,

    #[serde(rename = "Label")]
    pub label: Option<String>,

    #[serde(rename = "Title")]
    pub title: Option<String>,

    #[serde(rename = "Description")]
    pub description: Option<String>,

    #[serde(rename = "Url")]
    pub url: Option<String>,

    #[serde(rename = "Event")]
    pub event: Option<String>,

    #[serde(rename = "EventKey")]
    pub event_key: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BasicWechatMessage {
    #[serde(rename = "ToUserName")]
    pub to_user_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WechatEncryptMessage {
    #[serde(rename = "ToUserName")]
    pub to_user_name: String,
    #[serde(rename = "Encrypt")]
    pub encrypt: Option<String>,
}
