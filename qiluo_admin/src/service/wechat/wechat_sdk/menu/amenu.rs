use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct AccessTokenResponse {
    pub access_token: String,
    pub expires_in: u32,
}

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum MenuButton {
    #[serde(rename = "click")]
    Click { name: String, key: String },
    #[serde(rename = "view")]
    View { name: String, url: String },
    #[serde(rename = "miniprogram")]
    MiniProgram {
        name: String,
        url: String,
        appid: String,
        pagepath: String,
    },
}

#[derive(Serialize)]
pub struct MainMenuButton {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_button: Option<Vec<MenuButton>>,
    #[serde(flatten)]
    pub button_type: Option<MenuButton>,
}

#[derive(Serialize)]
pub struct CustomMenu {
    pub button: Vec<MainMenuButton>,
}
