use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize,Default)]
#[repr(i8)]
pub enum MessageMode {
    #[serde(rename = "plain")]
    #[default]
    Plain = 0,
    #[serde(rename = "compatible")]
    Compatible = 1,
    #[serde(rename = "encrypted")]
    Encrypted = 2,
} 

impl std::fmt::Display for MessageMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageMode::Plain => write!(f, "明文模式"),
            MessageMode::Compatible => write!(f, "兼容模式"),
            MessageMode::Encrypted => write!(f, "安全模式"),
        }
    }
}

impl FromStr for MessageMode {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
         match s {
            "plain" => Ok(MessageMode::Plain),
            "compatible" => Ok(MessageMode::Compatible),
            "encrypted" => Ok(MessageMode::Encrypted),
            _ => Err("无效的 MessageMode 类型".to_owned()),
        }
    }
}

impl MessageMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            MessageMode::Plain => "plain",
            MessageMode::Compatible => "compatible",
            MessageMode::Encrypted => "encrypted",
        }
    }
 

    pub fn description(&self) -> &'static str {
        match self {
            MessageMode::Plain => "消息明文传输，不加密",
            MessageMode::Compatible => "兼容模式，支持明文和加密",
            MessageMode::Encrypted => "消息加密传输，更安全",
        }
    }
}
