use anyhow::Result;
use entity::resource;
use entity::resource::{ActiveModel as ResourceModel, Entity as ResourceEntity};
use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, IntoActiveModel, QueryFilter};

use async_trait::async_trait;
use domain::{Repository, Resource};
use log::info;
use mockall::automock;
use uuid::Uuid;

#[derive(Debug)]
pub struct ResourceRepository {
    db: DbConn,
}

impl ResourceRepository {
    pub fn new(db: DbConn) -> Self {
        Self { db }
    }
}

#[automock]
#[async_trait]
impl Repository for ResourceRepository {
    type Type = Resource;

    async fn create(&self, item: Resource) -> Result<Resource> {
        info!("creating resource: {:?}", item);
        let result = ResourceModel::from(item).insert(&self.db).await?;
        Ok(result.into_active_model().into())
    }

    async fn update(&self, id: Uuid, item: Resource) -> Result<Resource> {
        info!("updating resource {}", id);
        let result = ResourceEntity::find_by_id(id).one(&self.db).await?;
        let model = result
            .ok_or_else(|| anyhow::Error::msg(format!("Entity with id {} doesn't exist", id)))?;
        let updated_model = model
            .into_active_model()
            .update_model(item)
            .save(&self.db)
            .await?;
        Ok(updated_model.into())
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Resource> {
        info!("getting resource by id: {}", id);
        let result = ResourceEntity::find_by_id(id).one(&self.db).await?;
        match result {
            Some(result) => Ok(result.into_active_model().into()),
            None => Err(anyhow::Error::msg(format!(
                "Entity with id {} doesn't exist",
                id
            ))),
        }
    }

    async fn get_by_key(&self, key: String) -> Result<Resource> {
        info!("getting resource by key: {}", key);
        let result = ResourceEntity::find()
            .filter(resource::Column::Key.eq(key.clone()))
            .one(&self.db)
            .await?;
        match result {
            Some(result) => Ok(result.into_active_model().into()),
            None => Err(anyhow::Error::msg(format!(
                "Entity with key {} doesn't exist",
                key
            ))),
        }
    }

    async fn get_all(&self) -> Result<Vec<Resource>> {
        info!("getting all resources");
        let cakes: Vec<entity::resource::Model> = ResourceEntity::find().all(&self.db).await?;
        Ok(cakes
            .into_iter()
            .map(|e| e.into_active_model().into())
            .collect())
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<()> {
        ResourceEntity::delete_many()
            .filter(resource::Column::Id.eq(id))
            .exec(&self.db)
            .await?;
        Ok(())
    }

    async fn delete_all(&self) -> Result<()> {
        ResourceEntity::delete_many().exec(&self.db).await?;
        Ok(())
    }
}
