use entity::resource;
use entity::resource::Entity as Resource;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220430_000001_create_resource_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                sea_query::Table::create()
                    .table(Resource)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(resource::Column::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(resource::Column::UserId).uuid())
                    .col(
                        ColumnDef::new(resource::Column::Key)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(resource::Column::Url).string(),
                    )
                    .col(ColumnDef::new(resource::Column::Tags).json())
                    .col(ColumnDef::new(resource::Column::Metadata).json())
                    .col(ColumnDef::new(resource::Column::CreatedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(resource::Column::UpdatedAt).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                sea_query::Index::create()
                    .name("idx__resources__key")
                    .table(Resource)
                    .col(resource::Column::Key)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                sea_query::Index::drop()
                    .name("idx__resources__key")
                    .table(Resource)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(sea_query::Table::drop().table(Resource).to_owned())
            .await
    }
}
