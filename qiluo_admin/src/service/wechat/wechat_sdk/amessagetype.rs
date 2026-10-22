use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum MessageType {
    Text,
    Image,
    Voice,
    Video,
    ShortVideo,
    Location,
    Link,
    Event,
    Music,
    News,
    Unknown(String),
}

impl From<&str> for MessageType {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "text" => MessageType::Text,
            "image" => MessageType::Image,
            "voice" => MessageType::Voice,
            "video" => MessageType::Video,
            "shortvideo" => MessageType::ShortVideo,
            "location" => MessageType::Location,
            "link" => MessageType::Link,
            "event" => MessageType::Event,
            "music" => MessageType::Music,
            "news" => MessageType::News,
            _ => MessageType::Unknown(s.to_string()),
        }
    }
}

impl Display for MessageType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            MessageType::Text => "text",
            MessageType::Image => "image",
            MessageType::Voice => "voice",
            MessageType::Video => "video",
            MessageType::ShortVideo => "shortvideo",
            MessageType::Location => "location",
            MessageType::Link => "link",
            MessageType::Event => "event",
            MessageType::Music => "music",
            MessageType::News => "news",
            MessageType::Unknown(s) => s,
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EventType {
    Subscribe,
    Unsubscribe,
    Scan,
    Location,
    Click,
    View,
    TemplateSendJobFinish,
    MassSendJobFinish,
    KfCreateSession,
    KfCloseSession,
    KfSwitchSession,
    CardPassCheck,
    CardNotPassCheck,
    UserGetCard,
    UserDelCard,
    Unknown(String),
}

impl From<&str> for EventType {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "subscribe" => EventType::Subscribe,
            "unsubscribe" => EventType::Unsubscribe,
            "scan" => EventType::Scan,
            "location" => EventType::Location,
            "click" => EventType::Click,
            "view" => EventType::View,
            "templatesendjobfinish" => EventType::TemplateSendJobFinish,
            "masssendjobfinish" => EventType::MassSendJobFinish,
            "kf_create_session" => EventType::KfCreateSession,
            "kf_close_session" => EventType::KfCloseSession,
            "kf_switch_session" => EventType::KfSwitchSession,
            "card_pass_check" => EventType::CardPassCheck,
            "card_not_pass_check" => EventType::CardNotPassCheck,
            "user_get_card" => EventType::UserGetCard,
            "user_del_card" => EventType::UserDelCard,
            _ => EventType::Unknown(s.to_string()),
        }
    }
}

impl Display for EventType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            EventType::Subscribe => "subscribe",
            EventType::Unsubscribe => "unsubscribe",
            EventType::Scan => "SCAN",
            EventType::Location => "LOCATION",
            EventType::Click => "CLICK",
            EventType::View => "VIEW",
            EventType::TemplateSendJobFinish => "TEMPLATESENDJOBFINISH",
            EventType::MassSendJobFinish => "MASSSENDJOBFINISH",
            EventType::KfCreateSession => "kf_create_session",
            EventType::KfCloseSession => "kf_close_session",
            EventType::KfSwitchSession => "kf_switch_session",
            EventType::CardPassCheck => "card_pass_check",
            EventType::CardNotPassCheck => "card_not_pass_check",
            EventType::UserGetCard => "user_get_card",
            EventType::UserDelCard => "user_del_card",
            EventType::Unknown(s) => s,
        };
        write!(f, "{}", s)
    }
}
