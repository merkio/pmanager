use entity::user;
use entity::user::Entity as User;
use sea_schema::migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220302_000001_create_user_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                sea_query::Table::create()
                    .table(User)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(user::Column::Uuid)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(user::Column::Name).string().not_null())
                    .col(ColumnDef::new(user::Column::Password).string().not_null())
                    .col(ColumnDef::new(user::Column::Role).string())
                    .col(ColumnDef::new(user::Column::CreatedAt).date_time())
                    .col(ColumnDef::new(user::Column::UpdatedAt).date_time())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(sea_query::Table::drop().table(User).to_owned())
            .await
    }
}
