use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use uuid::Uuid;

use crate::FileObject;

#[derive(Clone, Debug, Eq, Serialize, Deserialize)]
pub struct Resource {
    pub id: Option<Uuid>,
    pub key: String,
    pub url: Option<String>,
    pub tags: Option<Value>,
    pub user_id: Option<Uuid>,
    pub metadata: Option<Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[allow(dead_code)]
impl Resource {
    pub fn with_key(mut self, key: &str) -> Self {
        self.key = key.to_owned();
        self
    }

    pub fn with_tags(mut self, tags: Map<String, Value>) -> Self {
        self.tags = Some(Value::Object(tags));
        self
    }

    pub fn with_user_id(mut self, user_id: Uuid) -> Self {
        self.user_id = Some(user_id);
        self
    }

    pub fn with_metadata(mut self, metadata: Map<String, Value>) -> Self {
        self.metadata = Some(Value::Object(metadata));
        self
    }

    pub fn with_url(mut self, url: String) -> Self {
        self.url = Some(url);
        self
    }
}

pub fn from_file_object(object: &FileObject) -> Resource {
    Resource {
        id: None,
        url: object.url.to_owned(),
        key: object.key.to_owned(),
        tags: object.tags.to_owned(),
        user_id: object.user_id,
        metadata: object.metadata.to_owned(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

impl Default for Resource {
    fn default() -> Self {
        Self {
            id: None,
            key: "".to_owned(),
            url: None,
            tags: None,
            user_id: None,
            metadata: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
