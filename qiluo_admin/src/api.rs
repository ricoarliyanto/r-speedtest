pub mod sys_controll;
pub mod test;
pub mod wechat;
pub mod web_path;
pub mod net_tool_api;
pub mod net_scenario_api;
pub mod net_node_api;
pub mod net_alert_api;
pub mod net_ws_api;
pub mod tr143_api;
use crate::worker::invokefunction::{InvokeFunctionMsg, InvokeFunctionWorker};
use crate::worker::AppWorker;
use axum::Router;
use web_path::WebPath;
pub struct WebApi;

impl WebApi {
    pub fn routers() -> Router {
        let mut webpath = WebPath::new();

        webpath = webpath.merge(sys_controll::router_sys());
        webpath = webpath.merge(test::router_test());
        webpath = webpath.merge(wechat::router_wechat());
        webpath = webpath.merge(net_tool_api::router_net());
        webpath = webpath.merge(net_scenario_api::router_scenario());
        webpath = webpath.merge(net_node_api::router_node());
        webpath = webpath.merge(net_alert_api::router_alert());

        webpath = webpath.final_to_path();
        let expand_path = webpath.get_last_level_paths();

        let invfun = InvokeFunctionMsg {
            job_id: None,
            callfun: "updateapi".to_owned(),
            parmets: serde_json::to_string(&expand_path).unwrap(),
        };
        tokio::spawn(async move {
            let _ = InvokeFunctionWorker::execute_async(invfun).await;
        });
        let mut router = Router::new();
        for p in expand_path {
            if let Some(method_router) = p.method_router.clone() {
                router = router.route(&p.final_path, method_router);
            }
        }

        Router::new().merge(router)
    }
    pub fn white_routers() -> Router {
        Router::new()
            .merge(sys_controll::white_sys())
            .merge(test::white_test())
            .merge(wechat::white_wechat())
            .merge(net_ws_api::router_net_ws())
            .merge(tr143_api::router_tr143_public())
            .merge(net_node_api::router_node_public())
    }
}
