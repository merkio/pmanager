use sea_orm::entity::prelude::*;
use sea_orm::*;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uuid: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub role: Role,
}

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "role")]
pub enum Role {
    #[sea_orm(string_value = "USER")]
    USER,
    #[sea_orm(string_value = "ADMIN")]
    ADMIN,
    #[sea_orm(string_value = "GUEST")]
    GUEST,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {

    fn new() -> Self {
        Self {
            uuid: ActiveValue::Set(Uuid::new_v4()),
            created_at: ActiveValue::Set(Utc::now()),
            ..ActiveModelTrait::default()
        }
    }

    /// Will be triggered before insert / update
    fn before_save(mut self, _: bool) -> Result<Self, DbErr> {
        self.updated_at = ActiveValue::Set(Utc::now());
        Ok(self)
    }
}