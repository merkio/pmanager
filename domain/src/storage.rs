use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Storage {
    async fn create_bucket(&self, bucket: &str, location: &str) -> Result<()>;

    async fn delete_bucket(&self, bucket: &str) -> Result<()>;

    async fn list_objects(&self, bucket: &str) -> Result<Vec<String>>;

    async fn list_buckets(&self) -> Result<Vec<String>>;

    async fn upload_file(&self, bucket: &str, filename: &str, key: &str) -> Result<()>;

    async fn upload_object(&self, bucket: &str, file: &[u8], key: &str) -> Result<()>;

    async fn delete_object(&self, bucket: &str, key: &str) -> Result<()>;

    async fn delete_objects(&self, bucket: &str, keys: Vec<String>) -> Result<()>;
}

pub struct Object<'a> {
    pub filename: Option<String>,
    pub file: Option<&'a [u8]>,
    pub key: String,
}
