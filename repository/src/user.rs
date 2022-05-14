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
    fn new(db: DbConn) -> Self {
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

#[cfg(test)]
#[allow(unused_must_use)]
mod tests {

    use super::*;
    use anyhow::{anyhow, Result};
    use domain::User;
    use migration::Migrator;
    use sea_orm::Database;
    use sea_schema::migration::*;
    use std::collections::HashMap;
    use test_log::test;
    use testcontainers::{images::postgres::Postgres, *};
    use tokio::runtime::Runtime;

    async fn cleanup(db_url: &str) -> Result<()> {
        let repo = UserRepository::new(Database::connect(db_url).await?);
        repo.delete_all().await
    }

    #[test]
    fn run_tests() {
        let rt = Runtime::new().unwrap();
        let _ = env_logger::builder().is_test(true).try_init();

        let docker = clients::Cli::default();
        let postgres = docker.run(Postgres::default().with_version(14));
        let db_url = format!(
            "postgres://postgres@localhost:{}/postgres",
            postgres.get_host_port(5432).unwrap_or_default()
        );
        info!("DATABASE URL: {:?}", db_url);
        rt.block_on(async {
            Migrator::up(&Database::connect(db_url.clone()).await.unwrap(), None).await;
        });

        let mut results: HashMap<String, Result<()>> = HashMap::new();

        results.insert(
            "Get User by ID".to_owned(),
            rt.block_on(async { test_get_by_id(&db_url).await }),
        );
        rt.block_on(async { cleanup(&db_url).await });
        results.insert(
            "Insert User".to_owned(),
            rt.block_on(async { test_insert(&db_url).await }),
        );
        rt.block_on(async { cleanup(&db_url).await });
        results.insert(
            "Get all Users".to_owned(),
            rt.block_on(async { test_get_all(&db_url).await }),
        );
        rt.block_on(async { cleanup(&db_url).await });
        results.insert(
            "Update User".to_owned(),
            rt.block_on(async { test_update(&db_url).await }),
        );
        rt.block_on(async { cleanup(&db_url).await });

        for res in results.iter() {
            info!("TEST: '{}', RESULT: {:?}", res.0, res.1);
        }
        assert!(results.values().into_iter().all(|res| res.is_ok()));
    }

    async fn test_insert(db_url: &str) -> Result<()> {
        let input = User::default();
        let repo = UserRepository::new(Database::connect(db_url).await?);
        let res = repo.create(input).await;
        match res {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    async fn test_get_by_id(db_url: &str) -> Result<()> {
        let input = User::default();
        let repo = UserRepository::new(Database::connect(db_url).await?);
        let saved = repo.create(input).await?;
        let res = repo.get_by_id(saved.id.unwrap()).await;
        match res {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    async fn test_get_all(db_url: &str) -> Result<()> {
        let input = User::default();
        let repo = UserRepository::new(Database::connect(db_url).await?);
        let saved = repo.create(input).await?;
        let res = repo.get_all().await;
        match res {
            Ok(res) => {
                if res.contains(&saved) && res.len() == 1 {
                    Ok(())
                } else {
                    Err(anyhow!("Failed to save User"))
                }
            }
            Err(e) => Err(e),
        }
    }

    async fn test_update(db_url: &str) -> Result<()> {
        let input = User::default();
        let repo = UserRepository::new(Database::connect(db_url).await?);
        let mut saved = repo.create(input).await?;
        saved.name = "new name".to_owned();
        let updated = repo.update(saved.id.unwrap(), saved).await?;
        if updated.name != *"new name" {
            return Err(anyhow!("Failed to update User"));
        }
        Ok(())
    }
}
