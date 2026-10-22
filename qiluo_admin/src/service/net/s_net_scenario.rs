use crate::common::error::{Error, Result};
use crate::common::result::ApiResponse;
use crate::common::validatedjson::VJson;
use crate::common::validatedquery::VQuery;
use crate::db::{DB_READ, DB_WRITE};
use crate::model::net::net_scenario;
use crate::model::prelude::PageParams;
use crate::worker::net_scenario_worker::{NetScenarioJobMsg, NetScenarioWorker};
use crate::worker::AppWorker;
use axum::response::IntoResponse;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Validate, Debug)]
pub struct CreateScenarioReq {
    pub name: String,
    pub description: Option<String>,
    pub stop_on_failure: Option<bool>,
    pub execution_flow: serde_json::Value,
}

#[derive(Deserialize, Serialize, Validate, Debug)]
pub struct EditScenarioReq {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub stop_on_failure: Option<bool>,
    pub execution_flow: serde_json::Value,
}

#[derive(Deserialize, Serialize, Validate, Debug)]
pub struct ScenarioIdReq {
    pub id: String,
}

#[derive(Deserialize, Serialize, Validate, Debug)]
pub struct RunScenarioReq {
    pub scenario_id: String,
    pub target: String,
    pub node_id: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RunScenarioRes {
    pub execution_id: String,
    pub scenario_id: String,
    pub status: String,
}

pub async fn create_scenario(VJson(req): VJson<CreateScenarioReq>) -> impl IntoResponse {
    let db = DB_WRITE().await;
    let scenario_id = format!("scen_{}", Utc::now().timestamp_millis());

    let new_scen = net_scenario::ActiveModel {
        id: Set(scenario_id.clone()),
        name: Set(req.name),
        description: Set(req.description),
        stop_on_failure: Set(req.stop_on_failure.unwrap_or(true)),
        execution_flow: Set(req.execution_flow.to_string()),
        created_by: Set("admin".to_string()),
        created_at: Set(Utc::now().naive_utc()),
        updated_at: Set(Utc::now().naive_utc()),
    };

    if let Err(e) = new_scen.insert(db).await {
        return ApiResponse::<Option<String>>::from_result(Err(Error::Message(format!(
            "Failed to create scenario: {:?}",
            e
        ))));
    }

    ApiResponse::ok(scenario_id)
}

pub async fn edit_scenario(VJson(req): VJson<EditScenarioReq>) -> impl IntoResponse {
    let db = DB_WRITE().await;

    let scen_opt = match net_scenario::Entity::find()
        .filter(net_scenario::Column::Id.eq(&req.id))
        .one(db)
        .await
    {
        Ok(s) => s,
        Err(e) => return ApiResponse::<Option<String>>::from_result(Err(Error::Message(e.to_string()))),
    };

    if let Some(scen) = scen_opt {
        let mut active: net_scenario::ActiveModel = scen.into();
        active.name = Set(req.name);
        active.description = Set(req.description);
        active.stop_on_failure = Set(req.stop_on_failure.unwrap_or(true));
        active.execution_flow = Set(req.execution_flow.to_string());
        active.updated_at = Set(Utc::now().naive_utc());

        if let Err(e) = active.update(db).await {
            return ApiResponse::<Option<String>>::from_result(Err(Error::Message(e.to_string())));
        }
        ApiResponse::ok("Scenario updated")
    } else {
        ApiResponse::<Option<String>>::from_result(Err(Error::Message("Scenario not found".to_string())))
    }
}

pub async fn list_scenarios(VQuery(page_params): VQuery<PageParams>) -> impl IntoResponse {
    let db = DB_READ().await;
    let page = page_params.page_num.unwrap_or(1).max(1);
    let page_size = page_params.page_size.unwrap_or(20).min(100);

    let paginator = net_scenario::Entity::find()
        .order_by_desc(net_scenario::Column::CreatedAt)
        .paginate(db, page_size);

    let total = match paginator.num_items().await {
        Ok(t) => t,
        Err(e) => return ApiResponse::<Option<serde_json::Value>>::from_result(Err(Error::Message(e.to_string()))),
    };

    let list = match paginator.fetch_page(page - 1).await {
        Ok(l) => l,
        Err(e) => return ApiResponse::<Option<serde_json::Value>>::from_result(Err(Error::Message(e.to_string()))),
    };

    ApiResponse::ok(serde_json::json!({
        "list": list,
        "total": total,
        "page_num": page,
        "page_size": page_size
    }))
}

pub async fn get_scenario(VQuery(req): VQuery<ScenarioIdReq>) -> impl IntoResponse {
    let db = DB_READ().await;
    let id = req.id;

    let scen = match net_scenario::Entity::find()
        .filter(net_scenario::Column::Id.eq(&id))
        .one(db)
        .await
    {
        Ok(Some(s)) => s,
        Ok(None) => {
            return ApiResponse::<Option<net_scenario::Model>>::from_result(Err(Error::Message(format!(
                "Scenario {} not found",
                id
            ))))
        }
        Err(e) => return ApiResponse::<Option<net_scenario::Model>>::from_result(Err(Error::Message(e.to_string()))),
    };

    ApiResponse::ok(scen)
}

pub async fn delete_scenario(VJson(req): VJson<ScenarioIdReq>) -> impl IntoResponse {
    let db = DB_WRITE().await;
    let id = req.id;

    if let Err(e) = net_scenario::Entity::delete_many()
        .filter(net_scenario::Column::Id.eq(&id))
        .exec(db)
        .await
    {
        return ApiResponse::<Option<String>>::from_result(Err(Error::Message(e.to_string())));
    }

    ApiResponse::ok("Scenario deleted")
}

pub async fn run_scenario(VJson(req): VJson<RunScenarioReq>) -> impl IntoResponse {
    let db = DB_READ().await;
    let scenario_id = req.scenario_id.clone();

    // Verify scenario exists
    let scen_opt = match net_scenario::Entity::find()
        .filter(net_scenario::Column::Id.eq(&scenario_id))
        .one(db)
        .await
    {
        Ok(s) => s,
        Err(e) => {
            return ApiResponse::<Option<RunScenarioRes>>::from_result(Err(Error::Message(e.to_string())))
        }
    };

    if scen_opt.is_none() {
        return ApiResponse::<Option<RunScenarioRes>>::from_result(Err(Error::Message(format!(
            "Scenario {} not found",
            scenario_id
        ))));
    }

    let execution_id = format!("exec_{}_{}", scenario_id, Utc::now().timestamp_millis());

    // Enqueue scenario execution job to DragonflyDB
    let _ = NetScenarioWorker::enqueue_async(NetScenarioJobMsg {
        execution_id: execution_id.clone(),
        scenario_id: scenario_id.clone(),
        target: req.target,
        node_id: req.node_id,
    })
    .await;

    ApiResponse::ok(RunScenarioRes {
        execution_id,
        scenario_id,
        status: "queued".to_string(),
    })
}
