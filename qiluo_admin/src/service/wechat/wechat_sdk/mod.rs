pub mod amessagemode;
pub mod amessagetype;
pub mod awechat;
pub mod awechatcrypto;
pub mod material;
pub mod menu;
pub mod pay;
pub mod template;
pub mod wechatclient;

use chrono::Utc;
use sha1::{Digest, Sha1};

pub fn verify_signature(token: &str, timestamp: &str, nonce: &str, signature: &str) -> bool {
    let mut array = [token, timestamp, nonce];
    array.sort();
    let combined = array.join("");

    let mut hasher = Sha1::new();
    hasher.update(combined.as_bytes());
    let result = hasher.finalize();
    let hash = hex::encode(result);

    hash == signature
}

pub fn parse_basic_wechat_xml(xml: &str) -> Result<awechat::BasicWechatMessage, String> {
    match serde_xml_rs::from_str(xml) {
        Ok(message) => Ok(message),
        Err(e) => Err(format!("解析微信XML消息失败: {}", e)),
    }
}

pub fn parse_wechat_encry_xml(xml: &str) -> Result<awechat::WechatEncryptMessage, String> {
    match serde_xml_rs::from_str(xml) {
        Ok(message) => Ok(message),
        Err(e) => Err(format!("解析微信XML消息失败: {}", e)),
    }
}

pub fn parse_wechat_xml(xml: &str) -> Result<awechat::WechatMessage, String> {
    match serde_xml_rs::from_str(xml) {
        Ok(message) => Ok(message),
        Err(e) => Err(format!("解析微信XML消息失败: {}", e)),
    }
}

pub fn reply_wechat_xml(message: &awechat::WechatMessage, content: String) -> String {
    let time = Utc::now().timestamp();
    format!(
        "<xml>\n\
            <ToUserName><![CDATA[{to_user}]]></ToUserName>\n\
            <FromUserName><![CDATA[{from_user}]]></FromUserName>\n\
            <CreateTime>{time}</CreateTime>\n\
            <MsgType><![CDATA[text]]></MsgType>\n\
            <Content><![CDATA[{content}]]></Content>\n\
            </xml>",
        to_user = message.from_user_name,
        from_user = message.to_user_name,
        time = time,
        content = content
    )
}

/// 回复图片消息
pub fn reply_wechat_image_xml(message: &awechat::WechatMessage, media_id: &str) -> String {
    let time = Utc::now().timestamp();
    format!(
        "<xml>\n\
            <ToUserName><![CDATA[{to_user}]]></ToUserName>\n\
            <FromUserName><![CDATA[{from_user}]]></FromUserName>\n\
            <CreateTime>{time}</CreateTime>\n\
            <MsgType><![CDATA[image]]></MsgType>\n\
            <Image>\n\
                <MediaId><![CDATA[{media_id}]]></MediaId>\n\
            </Image>\n\
            </xml>",
        to_user = message.from_user_name,
        from_user = message.to_user_name,
        time = time,
        media_id = media_id
    )
}

/// 回复图文消息（单条）
pub fn reply_wechat_news_xml(
    message: &awechat::WechatMessage,
    title: &str,
    description: &str,
    pic_url: &str,
    url: &str,
) -> String {
    let time = Utc::now().timestamp();
    format!(
        "<xml>\n\
            <ToUserName><![CDATA[{to_user}]]></ToUserName>\n\
            <FromUserName><![CDATA[{from_user}]]></FromUserName>\n\
            <CreateTime>{time}</CreateTime>\n\
            <MsgType><![CDATA[news]]></MsgType>\n\
            <ArticleCount>1</ArticleCount>\n\
            <Articles>\n\
                <item>\n\
                    <Title><![CDATA[{title}]]></Title>\n\
                    <Description><![CDATA[{description}]]></Description>\n\
                    <PicUrl><![CDATA[{pic_url}]]></PicUrl>\n\
                    <Url><![CDATA[{url}]]></Url>\n\
                </item>\n\
            </Articles>\n\
            </xml>",
        to_user = message.from_user_name,
        from_user = message.to_user_name,
        time = time,
        title = title,
        description = description,
        pic_url = pic_url,
        url = url
    )
}

/// 回复音乐消息
pub fn reply_wechat_music_xml(
    message: &awechat::WechatMessage,
    title: &str,
    description: &str,
    music_url: &str,
    hq_music_url: &str,
    thumb_media_id: &str,
) -> String {
    let time = Utc::now().timestamp();
    format!(
        "<xml>\n\
            <ToUserName><![CDATA[{to_user}]]></ToUserName>\n\
            <FromUserName><![CDATA[{from_user}]]></FromUserName>\n\
            <CreateTime>{time}</CreateTime>\n\
            <MsgType><![CDATA[music]]></MsgType>\n\
            <Music>\n\
                <Title><![CDATA[{title}]]></Title>\n\
                <Description><![CDATA[{description}]]></Description>\n\
                <MusicUrl><![CDATA[{music_url}]]></MusicUrl>\n\
                <HQMusicUrl><![CDATA[{hq_music_url}]]></HQMusicUrl>\n\
                <ThumbMediaId><![CDATA[{thumb_media_id}]]></ThumbMediaId>\n\
            </Music>\n\
            </xml>",
        to_user = message.from_user_name,
        from_user = message.to_user_name,
        time = time,
        title = title,
        description = description,
        music_url = music_url,
        hq_music_url = hq_music_url,
        thumb_media_id = thumb_media_id
    )
}
