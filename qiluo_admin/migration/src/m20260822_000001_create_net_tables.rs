use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 1. net_tool Table
        let tool_table = Table::create()
            .table(NetTool::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(NetTool::Id)
                    .string_len(64)
                    .not_null()
                    .primary_key(),
            )
            .col(ColumnDef::new(NetTool::Name).string_len(128).not_null())
            .col(ColumnDef::new(NetTool::Category).string_len(64).not_null())
            .col(ColumnDef::new(NetTool::BinaryPath).string_len(255).not_null())
            .col(ColumnDef::new(NetTool::InputSchemaJson).text().not_null())
            .col(ColumnDef::new(NetTool::OutputSchemaJson).text().not_null())
            .col(
                ColumnDef::new(NetTool::DefaultTimeoutSec)
                    .integer()
                    .not_null()
                    .default(30),
            )
            .col(
                ColumnDef::new(NetTool::IsActive)
                    .boolean()
                    .not_null()
                    .default(true),
            )
            .col(
                ColumnDef::new(NetTool::RequiredPermission)
                    .string_len(128)
                    .not_null(),
            )
            .col(ColumnDef::new(NetTool::CreatedAt).date_time().not_null())
            .col(ColumnDef::new(NetTool::UpdatedAt).date_time().not_null())
            .to_owned();
        manager.create_table(tool_table).await?;

        // 2. net_node Table
        let node_table = Table::create()
            .table(NetNode::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(NetNode::Id)
                    .string_len(64)
                    .not_null()
                    .primary_key(),
            )
            .col(ColumnDef::new(NetNode::Name).string_len(128).not_null())
            .col(ColumnDef::new(NetNode::IpAddress).string_len(64).not_null())
            .col(ColumnDef::new(NetNode::Region).string_len(64).not_null())
            .col(
                ColumnDef::new(NetNode::Status)
                    .string_len(32)
                    .not_null()
                    .default("online"),
            )
            .col(ColumnDef::new(NetNode::LastHeartbeat).date_time().not_null())
            .col(ColumnDef::new(NetNode::Capabilities).text().null())
            .col(ColumnDef::new(NetNode::AuthTokenHash).string_len(255).null())
            .col(ColumnDef::new(NetNode::CreatedAt).date_time().not_null())
            .col(ColumnDef::new(NetNode::UpdatedAt).date_time().not_null())
            .to_owned();
        manager.create_table(node_table).await?;

        // 3. net_job Table
        let job_table = Table::create()
            .table(NetJob::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(NetJob::Id)
                    .string_len(64)
                    .not_null()
                    .primary_key(),
            )
            .col(ColumnDef::new(NetJob::ToolId).string_len(64).not_null())
            .col(ColumnDef::new(NetJob::NodeId).string_len(64).null())
            .col(ColumnDef::new(NetJob::ScenarioId).string_len(64).null())
            .col(ColumnDef::new(NetJob::TargetHost).string_len(255).not_null())
            .col(ColumnDef::new(NetJob::Parameters).text().not_null())
            .col(
                ColumnDef::new(NetJob::Status)
                    .string_len(32)
                    .not_null()
                    .default("pending"),
            )
            .col(ColumnDef::new(NetJob::RequestedBy).string_len(64).not_null())
            .col(ColumnDef::new(NetJob::CreatedAt).date_time().not_null())
            .col(ColumnDef::new(NetJob::StartedAt).date_time().null())
            .col(ColumnDef::new(NetJob::FinishedAt).date_time().null())
            .col(ColumnDef::new(NetJob::DurationSec).double().null())
            .to_owned();
        manager.create_table(job_table).await?;

        // 4. net_job_result Table
        let result_table = Table::create()
            .table(NetJobResult::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(NetJobResult::Id)
                    .string_len(64)
                    .not_null()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(NetJobResult::JobId)
                    .string_len(64)
                    .not_null()
                    .unique_key(),
            )
            .col(ColumnDef::new(NetJobResult::StructuredMetrics).text().null())
            .col(ColumnDef::new(NetJobResult::RawOutput).text().null())
            .col(ColumnDef::new(NetJobResult::ArtifactPath).string_len(512).null())
            .col(ColumnDef::new(NetJobResult::ExitCode).integer().null())
            .col(ColumnDef::new(NetJobResult::ErrorMessage).text().null())
            .col(ColumnDef::new(NetJobResult::CreatedAt).date_time().not_null())
            .to_owned();
        manager.create_table(result_table).await?;

        // 5. net_scenario Table
        let scenario_table = Table::create()
            .table(NetScenario::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(NetScenario::Id)
                    .string_len(64)
                    .not_null()
                    .primary_key(),
            )
            .col(ColumnDef::new(NetScenario::Name).string_len(128).not_null())
            .col(ColumnDef::new(NetScenario::Description).text().null())
            .col(
                ColumnDef::new(NetScenario::StopOnFailure)
                    .boolean()
                    .not_null()
                    .default(true),
            )
            .col(ColumnDef::new(NetScenario::ExecutionFlow).text().not_null())
            .col(ColumnDef::new(NetScenario::CreatedBy).string_len(64).not_null())
            .col(ColumnDef::new(NetScenario::CreatedAt).date_time().not_null())
            .col(ColumnDef::new(NetScenario::UpdatedAt).date_time().not_null())
            .to_owned();
        manager.create_table(scenario_table).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(NetScenario::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(NetJobResult::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(NetJob::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(NetNode::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(NetTool::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum NetTool {
    Table,
    Id,
    Name,
    Category,
    BinaryPath,
    InputSchemaJson,
    OutputSchemaJson,
    DefaultTimeoutSec,
    IsActive,
    RequiredPermission,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum NetNode {
    Table,
    Id,
    Name,
    IpAddress,
    Region,
    Status,
    LastHeartbeat,
    Capabilities,
    AuthTokenHash,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum NetJob {
    Table,
    Id,
    ToolId,
    NodeId,
    ScenarioId,
    TargetHost,
    Parameters,
    Status,
    RequestedBy,
    CreatedAt,
    StartedAt,
    FinishedAt,
    DurationSec,
}

#[derive(DeriveIden)]
pub enum NetJobResult {
    Table,
    Id,
    JobId,
    StructuredMetrics,
    RawOutput,
    ArtifactPath,
    ExitCode,
    ErrorMessage,
    CreatedAt,
}

#[derive(DeriveIden)]
pub enum NetScenario {
    Table,
    Id,
    Name,
    Description,
    StopOnFailure,
    ExecutionFlow,
    CreatedBy,
    CreatedAt,
    UpdatedAt,
}
