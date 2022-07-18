use chrono::{DateTime, Utc};
use domain::User;
use sea_orm::entity::prelude::*;
use sea_orm::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub role: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            id: ActiveValue::Set(Uuid::new_v4()),
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

impl From<User> for ActiveModel {
    fn from(user: User) -> Self {
        Self {
            id: ActiveValue::Set(user.id.unwrap_or_else(Uuid::new_v4)),
            name: ActiveValue::Set(user.name.clone()),
            email: ActiveValue::Set(user.email.clone()),
            password: ActiveValue::Set(user.password.clone()),
            enabled: ActiveValue::Set(user.enabled),
            role: ActiveValue::Set(user.role.to_string()),
            created_at: ActiveValue::Set(user.created_at),
            updated_at: ActiveValue::Set(user.updated_at),
        }
    }
}

impl From<ActiveModel> for User {
    fn from(model: ActiveModel) -> Self {
        User {
            id: Some(model.id.unwrap()),
            name: model.name.unwrap(),
            email: model.email.unwrap(),
            password: model.password.unwrap(),
            enabled: model.enabled.unwrap(),
            role: model.role.unwrap().parse().unwrap(),
            created_at: model.created_at.unwrap(),
            updated_at: model.updated_at.unwrap(),
        }
    }
}

impl ActiveModel {
    pub fn update_model(self, user: User) -> Self {
        Self {
            id: self.id,
            name: ActiveValue::Set(user.name.clone()),
            email: ActiveValue::Set(user.email.clone()),
            password: ActiveValue::Set(user.password.clone()),
            enabled: ActiveValue::Set(user.enabled),
            role: ActiveValue::Set(user.role.to_string()),
            created_at: ActiveValue::Set(user.created_at),
            updated_at: ActiveValue::Set(user.updated_at),
        }
    }
}
