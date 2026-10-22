use super::web_path::{WebPath, WebPathType};
use crate::service::net::s_net_scenario::*;
use axum::routing::{get, post};

pub fn router_scenario() -> WebPath {
    WebPath::new().nest(
        "/net/scenario",
        WebPath::new()
            .route(
                "/create",
                WebPathType::Post,
                Some("Create Diagnostic Scenario"),
                post(create_scenario),
            )
            .route(
                "/edit",
                WebPathType::Post,
                Some("Edit Diagnostic Scenario"),
                post(edit_scenario),
            )
            .route(
                "/list",
                WebPathType::Get,
                Some("List Diagnostic Scenarios"),
                get(list_scenarios),
            )
            .route(
                "/get",
                WebPathType::Get,
                Some("Get Scenario Detail"),
                get(get_scenario),
            )
            .route(
                "/delete",
                WebPathType::Post,
                Some("Delete Diagnostic Scenario"),
                post(delete_scenario),
            )
            .route(
                "/run",
                WebPathType::Post,
                Some("Run Diagnostic Scenario"),
                post(run_scenario),
            ),
    )
}
