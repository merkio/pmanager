use anyhow::Result;
use entity::user;
use entity::user::{ActiveModel as UserModel, Entity as UserEntity};
use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, IntoActiveModel, QueryFilter};

use async_trait::async_trait;
use domain::{Repository, User};
use log::info;
use mockall::automock;
use uuid::Uuid;

#[derive(Debug)]
pub struct UserRepository {
    db: DbConn,
}

impl UserRepository {
    pub fn new(db: DbConn) -> Self {
        Self { db }
    }
}

#[automock]
#[async_trait]
impl Repository for UserRepository {
    type Type = User;

    async fn create(&self, user: User) -> Result<User> {
        info!("creating User: {:?}", user);
        let result = UserModel::from(user).insert(&self.db).await?;
        Ok(result.into_active_model().into())
    }

    async fn update(&self, id: Uuid, user: User) -> Result<User> {
        info!("updating User {}", id);
        let result = UserEntity::find_by_id(id).one(&self.db).await?;
        let model = result
            .ok_or_else(|| anyhow::Error::msg(format!("Entity with id {} doesn't exist", id)))?;
        let updated_model = model
            .into_active_model()
            .update_model(user)
            .save(&self.db)
            .await?;
        Ok(updated_model.into())
    }

    async fn get_by_id(&self, id: Uuid) -> Result<User> {
        info!("getting User by id: {}", id);
        let result = UserEntity::find_by_id(id).one(&self.db).await?;
        match result {
            Some(result) => Ok(result.into_active_model().into()),
            None => Err(anyhow::Error::msg(format!(
                "Entity with id {} doesn't exist",
                id
            ))),
        }
    }

    async fn get_all(&self) -> Result<Vec<User>> {
        info!("getting all Users");
        let cakes: Vec<entity::user::Model> = UserEntity::find().all(&self.db).await?;
        Ok(cakes
            .into_iter()
            .map(|e| e.into_active_model().into())
            .collect())
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<()> {
        UserEntity::delete_many()
            .filter(user::Column::Id.eq(id))
            .exec(&self.db)
            .await?;
        Ok(())
    }

    async fn delete_all(&self) -> Result<()> {
        UserEntity::delete_many().exec(&self.db).await?;
        Ok(())
    }
}
