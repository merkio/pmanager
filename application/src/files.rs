use std::sync::Arc;

use anyhow::Result;
use app_config::ApplicationConfig;
use async_trait::async_trait;
use bytes::Bytes;
use domain::*;
use remote::DefaultStorage;
use repository::ResourceRepository;
use sea_orm::DbConn;

#[async_trait]
pub trait FileService {
    async fn upload(self, object: Box<FileObject>) -> Result<String>;
    async fn download(self, key: String) -> Result<FileObject>;
}

pub struct DefaultFileService {
    resources: Box<dyn Repository<Type = Resource> + Send + Sync>,
    storage: Box<dyn Storage + Send + Sync>,
    bucket: String,
    hostname: String,
}

impl DefaultFileService {
    pub fn new(config: &ApplicationConfig, db: Arc<DbConn>) -> Self {
        Self {
            resources: Box::new(ResourceRepository::new(db)),
            storage: Box::new(DefaultStorage::from_config(config.aws.clone())),
            bucket: config.aws.bucket.clone(),
            hostname: config.aws.endpoint.clone(),
        }
    }
}

#[async_trait]
impl FileService for DefaultFileService {
    async fn upload(self, object: Box<FileObject>) -> Result<String> {
        let empty = Bytes::new();
        let data = object.data.as_ref().unwrap_or(&empty);
        let resource = from_file_object(&object);
        let key = resource.key.clone();
        let url = format!("{}/{}/{}", self.hostname, self.bucket, key);
        self.storage
            .upload_object(self.bucket.as_str(), data, key.as_str())
            .await?;
        self.resources
            .create(resource.with_url(url.clone()))
            .await?;
        Ok(url)
    }

    async fn download(self, key: String) -> Result<FileObject> {
        let resource = self.resources.get_by_key(key.to_owned()).await?;
        Ok(FileObject {
            url: resource.url,
            key: resource.key,
            tags: resource.tags,
            user_id: resource.user_id,
            metadata: resource.metadata,
            data: None,
        })
    }
}
