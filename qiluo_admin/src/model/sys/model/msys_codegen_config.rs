pub use super::args::asys_codegen_config::*;
pub use super::entity::sys_codegen_config::{self, ActiveModel, Model as SysCodegenConfigModel};
use crate::model::prelude::*;

impl SysCodegenConfigModel {
    /// 根据 module_name + table_name 查询配置
    pub async fn get_config(module_name: &str, table_name: &str) -> Result<Option<String>> {
        let db = DB().await;
        let result = sys_codegen_config::Entity::find()
            .filter(sys_codegen_config::Column::ModuleName.eq(module_name))
            .filter(sys_codegen_config::Column::TableName.eq(table_name))
            .one(db)
            .await?;
        Ok(result.map(|r| r.config_json))
    }

    /// 保存或更新配置（upsert）
    pub async fn save_config(arg: CodegenConfigSave) -> Result<String> {
        let db = DB().await;

        // 先查是否存在
        let existing = sys_codegen_config::Entity::find()
            .filter(sys_codegen_config::Column::ModuleName.eq(&arg.module_name))
            .filter(sys_codegen_config::Column::TableName.eq(&arg.table_name))
            .one(db)
            .await?;

        if let Some(row) = existing {
            // 更新
            let mut amodel: sys_codegen_config::ActiveModel = row.into();
            amodel.config_json = Set(arg.config_json);
            amodel.updated_at = Set(Local::now().naive_local());
            amodel.update(db).await?;
        } else {
            // 插入
            let id = GID().await;
            let amodel = sys_codegen_config::ActiveModel {
                id: Set(id),
                module_name: Set(arg.module_name),
                table_name: Set(arg.table_name),
                config_json: Set(arg.config_json),
                created_at: Set(Local::now().naive_local()),
                updated_at: Set(Local::now().naive_local()),
            };
            amodel.insert(db).await?;
        }

        Ok("Success".to_string())
    }
}
