use ctor::{ctor, dtor};
use domain::*;
use log::info;
use migration::Migrator;
use once_cell::sync::OnceCell;
use repository::*;
use sea_orm::Database;
use sea_schema::migration::*;
use test_log::test;
use testcontainers::{clients::Cli, images::postgres::Postgres, *};
use tokio::runtime::Runtime;

static DOCKER: OnceCell<Cli> = OnceCell::new();
static DB_URL: OnceCell<String> = OnceCell::new();
static RT: OnceCell<Runtime> = OnceCell::new();
static CONTAINER: OnceCell<Container<Cli, Postgres>> = OnceCell::new();

#[ctor]
fn setup() {
    let rt = Runtime::new().unwrap();

    let docker = DOCKER.get_or_init(clients::Cli::default);
    CONTAINER
        .set(docker.run(Postgres::default().with_version(14)))
        .unwrap();
    let db_url = format!(
        "postgres://postgres@localhost:{}/postgres",
        CONTAINER
            .get()
            .unwrap()
            .get_host_port(5432)
            .unwrap_or_default()
    );
    info!("DATABASE URL: {:?}", db_url);
    rt.block_on(async {
        Migrator::up(&Database::connect(db_url.clone()).await.unwrap(), None)
            .await
            .unwrap();
    });
    DB_URL.set(db_url).unwrap();
    RT.set(rt).unwrap();

    let _ = env_logger::builder().is_test(true).try_init();
    info!("Database run and ready");
}

#[dtor]
fn teardown() {
    DOCKER.get().unwrap().stop(CONTAINER.get().unwrap().id());
    DOCKER.get().unwrap().rm(CONTAINER.get().unwrap().id());
}

// async fn cleanup(db_url: &str) {
//     let repo = ResourceRepository::new(Database::connect(db_url).await.unwrap());
//     repo.delete_all().await.unwrap();
//     let repo = UserRepository::new(Database::connect(db_url).await.unwrap());
//     repo.delete_all().await.unwrap();
// }

#[test(tokio::test)]
async fn insert_resource() {
    let input = Resource::default().with_key("INSERT");
    let resource_repo =
        ResourceRepository::new(Database::connect(DB_URL.get().unwrap()).await.unwrap());
    let res = resource_repo.create(input.clone()).await.unwrap();
    assert_eq!(res.key, input.key);
    assert!(res.id.is_some());
}

#[test(tokio::test)]
async fn get_resource_by_id() {
    let input = Resource::default().with_key("GET BY ID");
    let resource_repo =
        ResourceRepository::new(Database::connect(DB_URL.get().unwrap()).await.unwrap());
    let saved = resource_repo.create(input.clone()).await.unwrap();
    let res = resource_repo.get_by_id(saved.id.unwrap()).await.unwrap();
    assert_eq!(res.key, input.key);
    assert!(res.id.is_some());
}

#[test(tokio::test)]
async fn get_all_resources() {
    let input = Resource::default().with_key("GET ALL");
    let resource_repo =
        ResourceRepository::new(Database::connect(DB_URL.get().unwrap()).await.unwrap());
    let _ = resource_repo.create(input.clone()).await;
    let res = resource_repo.get_all().await.unwrap();
    assert!(!res.is_empty());
}

#[test(tokio::test)]
async fn update_resource() {
    let input = Resource::default().with_key("NEW KEY");
    let resource_repo =
        ResourceRepository::new(Database::connect(DB_URL.get().unwrap()).await.unwrap());
    let saved = resource_repo
        .create(input.clone())
        .await
        .unwrap()
        .with_key("updated key");
    let updated = resource_repo
        .update(saved.id.unwrap(), saved.clone())
        .await
        .unwrap();
    assert_eq!(saved.key, updated.key);
}

#[test(tokio::test)]
async fn insert_user() {
    let input = User::default().with_name("INSERT USER");
    let repo = UserRepository::new(Database::connect(DB_URL.get().unwrap()).await.unwrap());
    let res = repo.create(input.clone()).await.unwrap();
    assert_eq!(input.name, res.name);
}

#[test(tokio::test)]
async fn test_get_user_by_id() {
    let input = User::default().with_name("GET USER BY ID");
    let repo = UserRepository::new(Database::connect(DB_URL.get().unwrap()).await.unwrap());
    let saved = repo.create(input.clone()).await.unwrap();
    let res = repo.get_by_id(saved.id.unwrap()).await.unwrap();

    assert_eq!(input.name, res.name);
}

#[test(tokio::test)]
async fn test_get_all_users() {
    let input = User::default().with_name("GET ALL USERS");
    let repo = UserRepository::new(Database::connect(DB_URL.get().unwrap()).await.unwrap());
    repo.create(input.clone()).await.unwrap();
    let res = repo.get_all().await.unwrap();
    assert!(!res.is_empty());
}

#[test(tokio::test)]
async fn test_update_user() {
    let input = User::default().with_name("NEW USER");
    let repo = UserRepository::new(Database::connect(DB_URL.get().unwrap()).await.unwrap());
    let saved = repo
        .create(input.clone())
        .await
        .unwrap()
        .with_name("UPDATED USER");
    let updated = repo.update(saved.id.unwrap(), saved.clone()).await.unwrap();

    assert_eq!(saved.name, updated.name);
}
