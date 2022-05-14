mod resource;
mod s3;
mod user;

pub use resource::*;
pub use s3::*;
pub use user::*;

use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait Repository {
    type Type;

    async fn create(&self, item: Self::Type) -> Result<Self::Type>;
    async fn update(&self, id: Uuid, item: Self::Type) -> Result<Self::Type>;
    async fn get_by_id(&self, id: Uuid) -> Result<Self::Type>;
    async fn get_all(&self) -> Result<Vec<Self::Type>>;
    async fn delete_by_id(&self, id: Uuid) -> Result<()>;
    async fn delete_all(&self) -> Result<()>;
}
