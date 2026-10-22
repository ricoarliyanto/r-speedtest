use super::amenu::*;

#[derive(Default)]
pub struct MenuBuilder {
    buttons: Vec<MainMenuButton>,
}

impl MenuBuilder { 

    pub fn add_click_button(mut self, name: &str, key: &str) -> Self {
        self.buttons.push(MainMenuButton {
            name: name.to_string(),
            sub_button: None,
            button_type: Some(MenuButton::Click {
                name: name.to_string(),
                key: key.to_string(),
            }),
        });
        self
    }

    pub fn add_view_button(mut self, name: &str, url: &str) -> Self {
        self.buttons.push(MainMenuButton {
            name: name.to_string(),
            sub_button: None,
            button_type: Some(MenuButton::View {
                name: name.to_string(),
                url: url.to_string(),
            }),
        });
        self
    }

    pub fn add_miniprogram_button(
        mut self,
        name: &str,
        url: &str,
        appid: &str,
        pagepath: &str,
    ) -> Self {
        self.buttons.push(MainMenuButton {
            name: name.to_string(),
            sub_button: None,
            button_type: Some(MenuButton::MiniProgram {
                name: name.to_string(),
                url: url.to_string(),
                appid: appid.to_string(),
                pagepath: pagepath.to_string(),
            }),
        });
        self
    }

    pub fn add_submenu_button(mut self, name: &str, sub_buttons: Vec<MenuButton>) -> Self {
        self.buttons.push(MainMenuButton {
            name: name.to_string(),
            sub_button: Some(sub_buttons),
            button_type: None,
        });
        self
    }

    pub fn build(self) -> CustomMenu {
        CustomMenu {
            button: self.buttons,
        }
    }
}
