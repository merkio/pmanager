use chrono::{DateTime, Utc};
use domain::Resource;
use sea_orm::entity::prelude::*;
use sea_orm::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "resources")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub key: String,
    pub tags: Option<Value>,
    pub user_id: Option<Uuid>,
    pub metadata: Option<Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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

impl From<Resource> for ActiveModel {
    fn from(res: Resource) -> Self {
        Self {
            id: ActiveValue::Set(res.id.unwrap_or_else(Uuid::new_v4)),
            key: ActiveValue::Set(res.key.clone()),
            tags: ActiveValue::Set(res.tags),
            user_id: ActiveValue::Set(res.user_id),
            metadata: ActiveValue::Set(res.metadata),
            created_at: ActiveValue::Set(res.created_at),
            updated_at: ActiveValue::Set(res.updated_at),
        }
    }
}

impl From<ActiveModel> for Resource {
    fn from(model: ActiveModel) -> Self {
        Resource {
            id: Some(model.id.unwrap()),
            key: model.key.unwrap(),
            tags: model.tags.unwrap(),
            user_id: model.user_id.unwrap(),
            metadata: model.metadata.unwrap(),
            created_at: model.created_at.unwrap(),
            updated_at: model.updated_at.unwrap(),
        }
    }
}

impl ActiveModel {

    pub fn update_model(self, res: Resource) -> Self {
        Self {
            key: ActiveValue::Set(res.key.clone()),
            tags: ActiveValue::Set(res.tags.or_else(|| ActiveValue::unwrap(self.tags))),
            user_id: ActiveValue::Set(res.user_id.or_else(|| ActiveValue::unwrap(self.user_id))),
            metadata: ActiveValue::Set(res.metadata.or_else(|| ActiveValue::unwrap(self.metadata))),
            created_at: ActiveValue::Set(res.created_at),
            updated_at: ActiveValue::Set(res.updated_at),
            id: ActiveValue::Set(self.id.unwrap()),
        }
    }
}