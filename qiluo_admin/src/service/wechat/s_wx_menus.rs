use super::wechat_sdk::{menu::amenu::*, wechatclient::WeChatClient};
use crate::model::wechat::args::awx_menus::WxMenuAction;
use crate::model::wechat::model::{
    mwx_accounts::WxAccountsModel,
    mwx_menus::{WxMenusAdd, WxMenusDel, WxMenusEdit, WxMenusModel, WxMenusSearch},
};
use crate::model::prelude::*;
use crate::service::prelude::*; 
use futures::future::BoxFuture;

pub async fn list_tree(
    VQuery(arg): VQuery<PageParams>,
    VQuery(search): VQuery<WxMenusSearch>,
) -> impl IntoResponse {
    let rlist = WxMenusModel::list(arg, search).await;
    ApiResponse::from_result(rlist)
}
pub async fn edit(VJson(arg): VJson<WxMenusEdit>) -> impl IntoResponse {
    let r = WxMenusModel::edit(arg).await;
    ApiResponse::from_result(r)
}
pub async fn add(VJson(arg): VJson<WxMenusAdd>) -> impl IntoResponse {
    let r = WxMenusModel::add(arg).await;
    ApiResponse::from_result(r)
}
pub async fn delete(VQuery(arg): VQuery<WxMenusDel>) -> impl IntoResponse {
    let r = WxMenusModel::del(arg).await;
    ApiResponse::from_result(r)
}

/// 发布菜单到微信（将本地菜单数据推送到微信服务器）
pub async fn pull_menu(VJson(arg): VJson<WxMenuAction>) -> impl IntoResponse {
    let account = WxAccountsModel::find_by_id(arg.account_id).await;
    match account {
        Ok(Some(mp)) => {
            let client = WeChatClient::new(mp.app_id, mp.app_secret);
            // 从数据库读取该公众号的菜单，构建微信菜单结构
            let menus = WxMenusModel::get_all_by_account(arg.account_id).await;
            match menus {
                Ok(menu_list) => {
                    // 构建微信菜单结构
                    let custom_menu = build_custom_menu(&menu_list);
                    let result = client.create_menu(&custom_menu).await;
                    match result {
                        Ok(msg) => ApiResponse::ok(&msg),
                        Err(e) => ApiResponse::internal_server_error(e.to_string()),
                    }
                }
                Err(e) => ApiResponse::internal_server_error(e.to_string()),
            }
        }
        _ => ApiResponse::not_found("公众号不存在"),
    }
}

/// 从微信服务器同步菜单到本地
pub async fn sync_menu(VJson(arg): VJson<WxMenuAction>) -> impl IntoResponse {
    let account = WxAccountsModel::find_by_id(arg.account_id).await;
    match account {
        Ok(Some(mp)) => {
            let client = WeChatClient::new(mp.app_id, mp.app_secret);
            let result = client.get_menu().await;
            match result {
                Ok(menu_data) => {
                    // 先删除该公众号的旧菜单
                    let _ = WxMenusModel::delete_by_account(arg.account_id).await;
                    // 解析微信菜单并保存到数据库
                    let count = save_wechat_menu(arg.account_id, &menu_data).await;
                    match count {
                        Ok(n) => ApiResponse::ok(format!("同步成功，共 {} 个菜单项", n)),
                        Err(e) => ApiResponse::internal_server_error(e.to_string()),
                    }
                }
                Err(e) => ApiResponse::internal_server_error(e.to_string()),
            }
        }
        _ => ApiResponse::not_found("公众号不存在"),
    }
}

/// 删除微信服务器上的自定义菜单
pub async fn delete_remote_menu(VJson(arg): VJson<WxMenuAction>) -> impl IntoResponse {
    let account = WxAccountsModel::find_by_id(arg.account_id).await;
    match account {
        Ok(Some(mp)) => {
            let client = WeChatClient::new(mp.app_id, mp.app_secret);
            let result = client.delete_menu().await;
            match result {
                Ok(msg) => ApiResponse::ok(&msg),
                Err(e) => ApiResponse::internal_server_error(e.to_string()),
            }
        }
        _ => ApiResponse::not_found("公众号不存在"),
    }
}

/// 将数据库菜单构建为微信菜单格式
fn build_custom_menu(menu_list: &[(i64, Option<i64>, String, String, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, i32, i8)]) -> CustomMenu {
    let mut buttons: Vec<MainMenuButton> = Vec::new();
    
    // 找出顶级菜单（parent_id 为 None 或 0）
    let top_menus: Vec<_> = menu_list.iter()
        .filter(|m| m.1.is_none() || m.1 == Some(0))
        .collect();
    
    for top in top_menus.iter() {
        let (id, _, name, menu_type, key, url, _media_id, appid, pagepath, _sort_order, status) = top;
        if *status != 1 { continue; }
        
        // 查找子菜单（parent_id == 当前菜单 id）
        let sub_buttons: Vec<MenuButton> = menu_list.iter()
            .filter(|m| m.1 == Some(*id))
            .filter_map(|m| {
                if m.10 != 1 { return None; } // 跳过禁用
                match m.3.as_str() {
                    "click" => Some(MenuButton::Click {
                        name: m.2.clone(),
                        key: m.4.clone().unwrap_or_default(),
                    }),
                    "view" => Some(MenuButton::View {
                        name: m.2.clone(),
                        url: m.5.clone().unwrap_or_default(),
                    }),
                    "miniprogram" => Some(MenuButton::MiniProgram {
                        name: m.2.clone(),
                        url: m.5.clone().unwrap_or_default(),
                        appid: m.7.clone().unwrap_or_default(),
                        pagepath: m.8.clone().unwrap_or_default(),
                    }),
                    _ => Some(MenuButton::Click {
                        name: m.2.clone(),
                        key: m.4.clone().unwrap_or_default(),
                    }),
                }
            })
            .collect();

        // 顶级菜单有子菜单时不设 button_type
        let button_type = if sub_buttons.is_empty() {
            match menu_type.as_str() {
                "click" => Some(MenuButton::Click {
                    name: name.clone(),
                    key: key.clone().unwrap_or_default(),
                }),
                "view" => Some(MenuButton::View {
                    name: name.clone(),
                    url: url.clone().unwrap_or_default(),
                }),
                "miniprogram" => Some(MenuButton::MiniProgram {
                    name: name.clone(),
                    url: url.clone().unwrap_or_default(),
                    appid: appid.clone().unwrap_or_default(),
                    pagepath: pagepath.clone().unwrap_or_default(),
                }),
                _ => Some(MenuButton::Click {
                    name: name.clone(),
                    key: key.clone().unwrap_or_default(),
                }),
            }
        } else {
            None
        };
        
        buttons.push(MainMenuButton {
            name: name.clone(),
            sub_button: if sub_buttons.is_empty() { None } else { Some(sub_buttons) },
            button_type,
        });
    }
    
    CustomMenu { button: buttons }
}

/// 解析微信返回的菜单 JSON 并保存到数据库
async fn save_wechat_menu(account_id: i64, menu_data: &serde_json::Value) -> Result<usize> {
    let buttons = menu_data["menu"]["button"].as_array();
    let mut count = 0;
    
    if let Some(btns) = buttons {
        for (idx, btn) in btns.iter().enumerate() {
            count += save_menu_button(account_id, 0, idx as i32, btn).await?;
        }
    }
    
    Ok(count)
}

fn save_menu_button(account_id: i64, parent_id: i64, sort_order: i32, btn: &serde_json::Value) -> BoxFuture<'_, Result<usize>> {
    let name = btn["name"].as_str().unwrap_or("").to_string();
    let menu_type = if btn["type"].is_null() { "click".to_string() } else { btn["type"].as_str().unwrap_or("click").to_string() };
    let key = btn["key"].as_str().map(|s| s.to_string());
    let url = btn["url"].as_str().map(|s| s.to_string());
    let media_id = btn["media_id"].as_str().map(|s| s.to_string());
    let appid = btn["appid"].as_str().map(|s| s.to_string());
    let pagepath = btn["pagepath"].as_str().map(|s| s.to_string());
    let article_id = btn["article_id"].as_str().map(|s| s.to_string());

    let add_arg = WxMenusAdd {
        account_id,
        parent_id: if parent_id == 0 { None } else { Some(parent_id) },
        menu_name: name,
        menu_type,
        menu_key: key,
        url,
        media_id,
        appid,
        pagepath,
        article_id,
        sort_order: Some(sort_order),
        status: Some(1),
        created_at: None,
        updated_at: None,
    };

    Box::pin(async move {
        WxMenusModel::add(add_arg).await?;

        let mut count = 1;
        
        if let Some(sub_buttons) = btn["sub_button"].as_array() {
            for (idx, sub) in sub_buttons.iter().enumerate() {
                count += save_menu_button(account_id, 0, idx as i32, sub).await?;
            }
        }
        
        Ok(count)
    })
}
