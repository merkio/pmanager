use bytes::Bytes;
use serde::Serialize;
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct FileObject {
    pub key: String,
    pub url: Option<String>,
    pub tags: Option<Value>,
    pub user_id: Option<Uuid>,
    pub metadata: Option<Value>,
    pub data: Option<Bytes>,
}
