use anyhow::Result;
use async_trait::async_trait;
use domain::*;
use std::sync::Arc;

#[async_trait]
pub trait FileService {
    async fn upload(&self, object: FileObject) -> Result<String>
    where
        Self: Sized;
    async fn download(&self, key: &str) -> Result<FileObject>
    where
        Self: Sized;
}

pub struct DefaultFileService<'a> {
    resources: Arc<Box<dyn Repository<Type = Resource> + Send + Sync>>,
    storage: Arc<Box<dyn Storage + Send + Sync>>,
    bucket: &'a str,
}

impl <'a >DefaultFileService<'a> {
    pub fn new(
        resources: Arc<Box<dyn Repository<Type = Resource> + Send + Sync>>,
        storage: Arc<Box<dyn Storage + Send + Sync>>,
        bucket: &'a str,
    ) -> Self {
        DefaultFileService {
            resources,
            storage,
            bucket,
        }
    }
}

#[async_trait]
impl FileService for DefaultFileService<'_> {
    async fn upload(&self, object: FileObject) -> Result<String> {
        let data = &object.data;
        let resource = from_file_object(&object);
        let key = resource.key.clone();
        self.resources.create(resource).await?;
        self.storage
            .upload_object(self.bucket, data, key.as_str())
            .await
            .unwrap();
        Ok(key)
    }

    async fn download(&self, key: &str) -> Result<FileObject> {
        let resource = self.resources.get_by_key(key.to_owned()).await?;
        let data = self
            .storage
            .download_object(self.bucket, key)
            .await?;
        Ok(FileObject {
            key: resource.key,
            tags: resource.tags,
            user_id: resource.user_id,
            metadata: resource.metadata,
            data,
        })
    }
}
