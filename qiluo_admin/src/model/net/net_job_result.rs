use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "net_job_result")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    #[sea_orm(unique)]
    pub job_id: String,
    pub structured_metrics: Option<String>,
    pub raw_output: Option<String>,
    pub artifact_path: Option<String>,
    pub exit_code: Option<i32>,
    pub error_message: Option<String>,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::net_job::Entity",
        from = "Column::JobId",
        to = "super::net_job::Column::Id"
    )]
    Job,
}

impl Related<super::net_job::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Job.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
