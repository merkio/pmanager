use anyhow::Result;
use async_trait::async_trait;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{
    model::{BucketLocationConstraint, CreateBucketConfiguration},
    types::ByteStream,
    Client, Credentials,
};
use aws_smithy_http::endpoint::Endpoint;
use aws_types::{credentials::SharedCredentialsProvider, region::Region};
use app_config::AwsConfig;
use domain::Storage;
use http::Uri;
use log::info;
use std::{path::Path, str::FromStr};

#[derive(Debug, Clone)]
pub struct DefaultStorage {
    client: Client,
}

impl DefaultStorage {
    pub fn new(client: Client) -> Self {
        DefaultStorage { client }
    }

    pub async fn from_env() -> Self {
        let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
        let config = aws_config::from_env().region(region_provider).load().await;
        let client = Client::new(&config);

        DefaultStorage { client }
    }

    pub async fn from_config(config: AwsConfig) -> Self {
        let mut s3_config_builder = aws_sdk_s3::config::Builder::new();
        s3_config_builder = s3_config_builder
            .endpoint_resolver(Endpoint::immutable(
                Uri::from_str(&config.endpoint).unwrap(),
            ))
            .region(Some(Region::new(config.region.clone())))
            .credentials_provider(SharedCredentialsProvider::new(Credentials::from_keys(
                config.access_key_id.clone(),
                config.secret_access_key,
                None,
            )));
        let client = Client::from_conf(s3_config_builder.build());

        DefaultStorage { client }
    }
}

#[async_trait]
impl Storage for DefaultStorage {
    async fn create_bucket(&self, bucket: &str, location: &str) -> Result<()> {
        info!("Create bucket: {}", bucket);
        self.client
            .create_bucket()
            .create_bucket_configuration(
                CreateBucketConfiguration::builder()
                    .location_constraint(BucketLocationConstraint::from(location))
                    .build(),
            )
            .bucket(bucket)
            .send()
            .await?;
        Ok(())
    }

    async fn delete_bucket(&self, bucket: &str) -> Result<()> {
        info!("Remove bucket: {}", bucket);
        self.client.delete_bucket().bucket(bucket).send().await?;
        Ok(())
    }

    async fn list_objects(&self, bucket: &str) -> Result<Vec<String>> {
        info!("List of objects in bucket: {}", bucket);
        let resp = self.client.list_objects_v2().bucket(bucket).send().await?;
        Ok(resp
            .contents
            .unwrap_or_default()
            .iter()
            .map(|obj| obj.key.as_deref().unwrap_or_default().to_string())
            .collect::<Vec<String>>())
    }

    async fn upload_file(&self, bucket: &str, filename: &str, key: &str) -> Result<()> {
        info!(
            "Upload file: {} into bucket: {} with key: {}",
            filename, bucket, key
        );
        let body = ByteStream::from_path(Path::new(filename)).await?;
        info!("Upload file from path: {:?}", body);
        self.client
            .put_object()
            .bucket(bucket)
            .key(key)
            .body(body)
            .send()
            .await?;

        Ok(())
    }

    async fn upload_object(&self, bucket: &str, file: &[u8], key: &str) -> Result<()> {
        info!("Upload object into bucket: {} with key: {}", bucket, key);
        let body = ByteStream::from(file.to_vec());
        self.client
            .put_object()
            .bucket(bucket)
            .key(key)
            .body(body)
            .send()
            .await?;

        Ok(())
    }

    async fn delete_object(&self, bucket: &str, key: &str) -> Result<()> {
        info!("Delete object from bucket: {} with key: {}", bucket, key);
        self.client
            .delete_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await?;

        Ok(())
    }

    async fn delete_objects(&self, bucket: &str, keys: Vec<String>) -> Result<()> {
        for key in keys.iter() {
            self.delete_object(bucket, key).await?;
        }

        Ok(())
    }

    async fn list_buckets(&self) -> Result<Vec<String>> {
        info!("List of buckets");
        let resp = self.client.list_buckets().send().await?;
        Ok(resp
            .buckets
            .unwrap_or_default()
            .iter()
            .map(|bucket| bucket.name().unwrap_or_default().to_string())
            .collect::<Vec<String>>())
    }
}
