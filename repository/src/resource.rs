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
    fn new(db: DbConn) -> Self {
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

#[cfg(test)]
#[allow(unused_must_use)]
mod tests {

    use super::*;
    use anyhow::{anyhow, Result};
    use domain::Resource;
    use migration::Migrator;
    use sea_orm::Database;
    use sea_schema::migration::*;
    use std::collections::HashMap;
    use test_log::test;
    use testcontainers::{images::postgres::Postgres, *};
    use tokio::runtime::Runtime;

    async fn cleanup(db_url: &str) -> Result<()> {
        let repo = ResourceRepository::new(Database::connect(db_url).await?);
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
            "Get Resource by ID".to_owned(),
            rt.block_on(async { test_get_by_id(&db_url).await }),
        );
        rt.block_on(async { cleanup(&db_url).await });
        results.insert(
            "Insert Resource".to_owned(),
            rt.block_on(async { test_insert(&db_url).await }),
        );
        rt.block_on(async { cleanup(&db_url).await });
        results.insert(
            "Get all Resources".to_owned(),
            rt.block_on(async { test_get_all(&db_url).await }),
        );
        rt.block_on(async { cleanup(&db_url).await });
        results.insert(
            "Update Resource".to_owned(),
            rt.block_on(async { test_update(&db_url).await }),
        );
        rt.block_on(async { cleanup(&db_url).await });

        for res in results.iter() {
            info!("TEST: '{}', RESULT: {:?}", res.0, res.1);
        }
        assert!(results.values().into_iter().all(|res| res.is_ok()));
    }

    async fn test_insert(db_url: &str) -> Result<()> {
        let input = Resource::default();
        let resource_repo = ResourceRepository::new(Database::connect(db_url).await?);
        let res = resource_repo.create(input).await;
        match res {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    async fn test_get_by_id(db_url: &str) -> Result<()> {
        let input = Resource::default();
        let resource_repo = ResourceRepository::new(Database::connect(db_url).await?);
        let saved = resource_repo.create(input).await?;
        let res = resource_repo.get_by_id(saved.id.unwrap()).await;
        match res {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    async fn test_get_all(db_url: &str) -> Result<()> {
        let input = Resource::default();
        let resource_repo = ResourceRepository::new(Database::connect(db_url).await?);
        let saved = resource_repo.create(input).await?;
        let res = resource_repo.get_all().await;
        match res {
            Ok(res) => {
                if res.contains(&saved) && res.len() == 1 {
                    Ok(())
                } else {
                    Err(anyhow!("Failed to save resource"))
                }
            }
            Err(e) => Err(e),
        }
    }

    async fn test_update(db_url: &str) -> Result<()> {
        let input = Resource::default();
        let resource_repo = ResourceRepository::new(Database::connect(db_url).await?);
        let mut saved = resource_repo.create(input).await?;
        saved.key = "new key".to_owned();
        let updated = resource_repo.update(saved.id.unwrap(), saved).await?;
        if updated.key != *"new key" {
            return Err(anyhow!("Failed to update resource"));
        }
        Ok(())
    }
}
