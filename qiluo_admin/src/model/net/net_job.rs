use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "net_job")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub tool_id: String,
    pub node_id: Option<String>,
    pub scenario_id: Option<String>,
    pub target_host: String,
    pub parameters: String,
    pub status: String,
    pub requested_by: String,
    pub created_at: DateTime,
    pub started_at: Option<DateTime>,
    pub finished_at: Option<DateTime>,
    pub duration_sec: Option<f64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::net_tool::Entity",
        from = "Column::ToolId",
        to = "super::net_tool::Column::Id"
    )]
    Tool,
    #[sea_orm(
        belongs_to = "super::net_node::Entity",
        from = "Column::NodeId",
        to = "super::net_node::Column::Id"
    )]
    Node,
    #[sea_orm(
        belongs_to = "super::net_scenario::Entity",
        from = "Column::ScenarioId",
        to = "super::net_scenario::Column::Id"
    )]
    Scenario,
    #[sea_orm(has_one = "super::net_job_result::Entity")]
    Result,
}

impl Related<super::net_tool::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tool.def()
    }
}

impl Related<super::net_node::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Node.def()
    }
}

impl Related<super::net_scenario::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Scenario.def()
    }
}

impl Related<super::net_job_result::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Result.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
