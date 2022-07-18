use app_config::AwsConfig;
use domain::*;
use log::info;
use remote::*;
use test_log::test;
use testcontainers::*;

#[test(tokio::test)]
async fn create_bucket() {
    let _ = env_logger::builder().is_test(true).try_init();
    let image = images::minio::MinIO::default();
    let docker = clients::Cli::default();
    let container = docker.run(image);
    info!("Minio started...");
    let port = container.get_host_port_ipv4(9000);

    let config = config(port);
    let client = DefaultStorage::from_config(config.clone());

    let bucket_name = "new-bucket".to_owned();
    client
        .create_bucket(&bucket_name, &config.region.clone())
        .await
        .unwrap();

    let buckets = client.list_buckets().await.unwrap();
    assert!(!buckets.is_empty());
    assert!(buckets.contains(&bucket_name));
}

// #[test(tokio::test)]
// async fn create_object_in_bucket() {
//     let _ = env_logger::builder().is_test(true).try_init();
//     let image = images::minio::MinIO::default();
//     let docker = clients::Cli::default();
//     let container = docker.run(image);
//     info!("Minio started...");
//     let port = container.get_host_port_ipv4(9000);

//     let config = config(port);
//     let client = DefaultStorage::from_config(config.clone());

//     let bucket_name = "create-object-bucket".to_owned();
//     let key = "create-object-key.txt".to_owned();
//     client
//         .create_bucket(&bucket_name, &config.region.clone())
//         .await
//         .unwrap();
//     client
//         .upload_file(&bucket_name, "./test_file.txt", &key)
//         .await
//         .unwrap();

//     let objects = client.list_objects(&bucket_name).await.unwrap();
//     assert!(!objects.is_empty());
//     assert!(objects.contains(&key));
// }

// #[test(tokio::test)]
// async fn create_file_in_bucket() {
//     let _ = env_logger::builder().is_test(true).try_init();
//     let image = images::minio::MinIO::default();
//     let docker = clients::Cli::default();
//     let container = docker.run(image);
//     info!("Minio started...");
//     let port = container.get_host_port_ipv4(9000);

//     let config = config(port);
//     let client = DefaultStorage::from_config(config.clone());

//     let bucket_name = "create-file-bucket".to_owned();
//     let key = "create-file-key.txt".to_owned();
//     client
//         .create_bucket(&bucket_name, &config.region.clone())
//         .await
//         .unwrap();
//     client
//         .upload_object(&bucket_name, "Some stuff".as_bytes(), &key)
//         .await
//         .unwrap();

//     let objects = client.list_objects(&bucket_name).await.unwrap();
//     assert!(!objects.is_empty());
//     assert!(objects.contains(&key));
// }

// #[test(tokio::test)]
// async fn rm_bucket() {
//     let _ = env_logger::builder().is_test(true).try_init();
//     let image = images::minio::MinIO::default();
//     let docker = clients::Cli::default();
//     let container = docker.run(image);
//     info!("Minio started...");
//     let port = container.get_host_port_ipv4(9000);

//     let config = config(port);
//     let client = DefaultStorage::from_config(config.clone());

//     let bucket_name = "rm-bucket".to_owned();
//     client
//         .create_bucket(&bucket_name, &config.region.clone())
//         .await
//         .unwrap();
//     client.delete_bucket(&bucket_name).await.unwrap();

//     let buckets = client.list_buckets().await.unwrap();
//     assert!(!buckets.contains(&bucket_name));
// }

fn config(port: u16) -> AwsConfig {
    AwsConfig {
        access_key_id: String::from("minioadmin"),
        secret_access_key: String::from("minioadmin"),
        region: String::from("eu-west-1"),
        endpoint: format!("http://127.0.0.1:{}", port),
        bucket: String::from("test"),
    }
}
