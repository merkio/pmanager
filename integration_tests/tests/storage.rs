use config::AwsConfig;
use ctor::{ctor, dtor};
use domain::*;
use log::info;
use once_cell::sync::OnceCell;
use remote::*;
use test_log::test;
use testcontainers::{
    clients::Cli,
    core::Port,
    images::generic::{GenericImage, WaitFor},
    *,
};

static DOCKER: OnceCell<Cli> = OnceCell::new();
static CONTAINER: OnceCell<Container<Cli, GenericImage>> = OnceCell::new();

#[ctor]
fn setup() {
    let _ = env_logger::builder().is_test(true).try_init();
    let image = GenericImage::new("localstack/localstack")
        .with_env_var("SERVICES", "s3")
        .with_env_var("DEBUG", "1")
        .with_wait_for(WaitFor::message_on_stdout(
            "Running on https://0.0.0.0:4566",
        ));
    let docker = DOCKER.get_or_init(clients::Cli::default);

    CONTAINER
        .set(
            docker.run_with_args(
                image,
                RunArgs::default()
                    .with_name("localstack")
                    .with_mapped_port::<Port>((4566u16, 4566u16).into())
                    .with_mapped_port::<Port>((4571u16, 4571u16).into()),
            ),
        )
        .unwrap();
    info!("Localstack run and ready");
}

#[dtor]
fn teardown() {
    DOCKER.get().unwrap().stop(CONTAINER.get().unwrap().id());
    DOCKER.get().unwrap().rm(CONTAINER.get().unwrap().id());
}

#[test(tokio::test)]
async fn create_bucket() {
    let config = config();
    let client = DefaultStorage::from_config(config.clone()).await;

    let bucket_name = "new-bucket".to_owned();
    client
        .create_bucket(&bucket_name, &config.region.clone())
        .await
        .unwrap();

    let buckets = client.list_buckets().await.unwrap();
    assert!(!buckets.is_empty());
    assert!(buckets.contains(&bucket_name));
}

#[test(tokio::test)]
async fn create_object_in_bucket() {
    let config = config();
    let client = DefaultStorage::from_config(config.clone()).await;

    let bucket_name = "create-object-bucket".to_owned();
    let key = "create-object-key.txt".to_owned();
    client
        .create_bucket(&bucket_name, &config.region.clone())
        .await
        .unwrap();
    client
        .upload_file(&bucket_name, "./test_file.txt", &key)
        .await
        .unwrap();

    let objects = client.list_objects(&bucket_name).await.unwrap();
    assert!(!objects.is_empty());
    assert!(objects.contains(&key));
}

#[test(tokio::test)]
async fn create_file_in_bucket() {
    let config = config();
    let client = DefaultStorage::from_config(config.clone()).await;

    let bucket_name = "create-file-bucket".to_owned();
    let key = "create-file-key.txt".to_owned();
    client
        .create_bucket(&bucket_name, &config.region.clone())
        .await
        .unwrap();
    client
        .upload_object(&bucket_name, "Some stuff".as_bytes(), &key)
        .await
        .unwrap();

    let objects = client.list_objects(&bucket_name).await.unwrap();
    assert!(!objects.is_empty());
    assert!(objects.contains(&key));
}

#[test(tokio::test)]
async fn rm_bucket() {
    let config = config();
    let client = DefaultStorage::from_config(config.clone()).await;

    let bucket_name = "rm-bucket".to_owned();
    client
        .create_bucket(&bucket_name, &config.region.clone())
        .await
        .unwrap();
    client.delete_bucket(&bucket_name).await.unwrap();

    let buckets = client.list_buckets().await.unwrap();
    assert!(!buckets.contains(&bucket_name));
}

fn config() -> AwsConfig {
    AwsConfig {
        access_key_id: String::from("access_key"),
        secret_access_key: String::from("secret_key"),
        region: String::from("eu-west-1"),
        endpoint: String::from("http://127.0.0.1:4566"),
        bucket: String::from("test"),
    }
}
