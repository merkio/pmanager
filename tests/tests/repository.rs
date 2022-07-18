use domain::*;
use log::info;
use migration::{sea_orm::Database, Migrator, MigratorTrait};
use once_cell::sync::OnceCell;
use repository::ResourceRepository;
use std::sync::Arc;
use test_log::test;
use testcontainers::{clients::Cli, images::postgres::Postgres, *};

static DB_URL: OnceCell<String> = OnceCell::new();
static DOCKER: OnceCell<Arc<Cli>> = OnceCell::new();

fn init() {
    DOCKER.set(Arc::new(Cli::default())).unwrap();
}

// #[ctor::ctor]
async fn setup() -> Container<'static, Postgres> {
    init();
    let container = DOCKER
        .get()
        .unwrap()
        .run(RunnableImage::from(Postgres::default()).with_tag("14-alpine"));

    let db_url = format!(
        "postgres://postgres@localhost:{}/postgres",
        container.get_host_port_ipv4(5432)
    );
    info!("DATABASE URL: {:?}", db_url);
    Migrator::up(&Database::connect(db_url.clone()).await.unwrap(), None)
        .await
        .unwrap();
    DB_URL.set(db_url).unwrap();

    let _ = env_logger::builder().is_test(true).try_init();
    info!("Database image {} run and ready", container.id());
    container
}

#[test(tokio::test)]
async fn insert_resource() {
    let container = setup().await;
    info!("Container ID: {}", container.id());

    let input = Resource::default().with_key("INSERT");
    let resource_repo = ResourceRepository::new(Arc::new(
        Database::connect(DB_URL.get().unwrap()).await.unwrap(),
    ));
    let res = resource_repo.create(input.clone()).await.unwrap();
    assert_eq!(res.key, input.key);
    assert!(res.id.is_some());
}

// #[test(tokio::test)]
// async fn get_resource_by_id() {
//     let container = setup().await;
//     info!("Container ID: {}", container.id());

//     let input = Resource::default().with_key("GET BY ID");
//     let resource_repo = ResourceRepository::new(Arc::new(
//         Database::connect(DB_URL.get().unwrap()).await.unwrap(),
//     ));
//     let saved = resource_repo.create(input.clone()).await.unwrap();
//     let res = resource_repo.get_by_id(saved.id.unwrap()).await.unwrap();
//     assert_eq!(res.key, input.key);
//     assert!(res.id.is_some());
// }

// #[test(tokio::test)]
// async fn get_all_resources() {
//     let container = setup().await;
//     info!("Container ID: {}", container.id());

//     let input = Resource::default().with_key("GET ALL");
//     let resource_repo = ResourceRepository::new(Arc::new(
//         Database::connect(DB_URL.get().unwrap()).await.unwrap(),
//     ));
//     let _ = resource_repo.create(input.clone()).await;
//     let res = resource_repo.get_all().await.unwrap();
//     assert!(!res.is_empty());
// }

// #[test(tokio::test)]
// async fn update_resource() {
//     let container = setup().await;
//     info!("Container ID: {}", container.id());

//     let input = Resource::default().with_key("NEW KEY");
//     let resource_repo = ResourceRepository::new(Arc::new(
//         Database::connect(DB_URL.get().unwrap()).await.unwrap(),
//     ));
//     let saved = resource_repo
//         .create(input.clone())
//         .await
//         .unwrap()
//         .with_key("updated key");
//     let updated = resource_repo
//         .update(saved.id.unwrap(), saved.clone())
//         .await
//         .unwrap();
//     assert_eq!(saved.key, updated.key);
// }

// #[test(tokio::test)]
// async fn insert_user() {
//     let container = setup().await;
//     info!("Container ID: {}", container.id());

//     let input = User::default().with_name("INSERT USER");
//     let repo = UserRepository::new(Database::connect(DB_URL.get().unwrap()).await.unwrap());
//     let res = repo.create(input.clone()).await.unwrap();
//     assert_eq!(input.name, res.name);
// }

// #[test(tokio::test)]
// async fn test_get_user_by_id() {
//     let container = setup().await;
//     info!("Container ID: {}", container.id());

//     let input = User::default().with_name("GET USER BY ID");
//     let repo = UserRepository::new(Database::connect(DB_URL.get().unwrap()).await.unwrap());
//     let saved = repo.create(input.clone()).await.unwrap();
//     let res = repo.get_by_id(saved.id.unwrap()).await.unwrap();

//     assert_eq!(input.name, res.name);
// }

// #[test(tokio::test)]
// async fn test_get_all_users() {
//     let container = setup().await;
//     info!("Container ID: {}", container.id());

//     let input = User::default().with_name("GET ALL USERS");
//     let repo = UserRepository::new(Database::connect(DB_URL.get().unwrap()).await.unwrap());
//     repo.create(input.clone()).await.unwrap();
//     let res = repo.get_all().await.unwrap();
//     assert!(!res.is_empty());
// }

// #[test(tokio::test)]
// async fn test_update_user() {
//     let container = setup().await;
//     info!("Container ID: {}", container.id());

//     let input = User::default().with_name("NEW USER");
//     let repo = UserRepository::new(Database::connect(DB_URL.get().unwrap()).await.unwrap());
//     let saved = repo
//         .create(input.clone())
//         .await
//         .unwrap()
//         .with_name("UPDATED USER");
//     let updated = repo.update(saved.id.unwrap(), saved.clone()).await.unwrap();

//     assert_eq!(saved.name, updated.name);
// }
