use anyhow::Error;
use app_config::ApplicationConfig;
use application::{DefaultFileService, FileService};
use axum::{
    extract::{Extension, Multipart, Path},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use bytes::Bytes;
use domain::FileObject;
use log::info;
use sea_orm::DbConn;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;

pub fn files_routers() -> Router {
    Router::new()
        .route("/download/:key", get(download))
        .route("/upload", post(upload))
}

async fn upload(
    Extension(ref config): Extension<Arc<ApplicationConfig>>,
    Extension(ref db): Extension<Arc<DbConn>>,
    mut multipart: Multipart,
) -> Result<Json<UploadResponse>, ApiError> {
    info!("Received multipart request: \n{:?}", multipart);
    let file_service = get_file_service(config, db.clone());
    let mut tags: Option<Value> = None;
    let mut metadata: Option<Value> = None;
    let mut ignored_fields = vec![];
    let mut key = "Unknown".to_owned();
    let mut data = Bytes::new();
    while let Some(field) = multipart.next_field().await.unwrap() {
        match field.name().unwrap_or("no name") {
            "key" => key = field.text().await.unwrap_or(key),
            "tags" => {
                tags = field
                    .text()
                    .await
                    .ok()
                    .and_then(|s| serde_json::from_str(s.as_str()).ok())
            }
            "metadata" => {
                metadata = field
                    .text()
                    .await
                    .ok()
                    .and_then(|s| serde_json::from_str(s.as_str()).ok())
            }
            "file" => {
                data = field.bytes().await.unwrap();
            }
            name => ignored_fields.push(name.to_owned()),
        }
    }
    let url = file_service
        .upload(Box::new(FileObject {
            key: key.clone(),
            url: None,
            tags,
            metadata,
            user_id: None,
            data: Some(data),
        }))
        .await?;

    Ok(Json(UploadResponse {
        url,
        ignored_fields,
    }))
}

async fn download(
    Path(key): Path<String>,
    Extension(ref config): Extension<Arc<ApplicationConfig>>,
    Extension(ref db): Extension<Arc<DbConn>>,
) -> Result<Json<FileObject>, ApiError> {
    info!("Download file with key: {}", key);
    let file_service = get_file_service(config, db.clone());
    file_service
        .download(key)
        .await
        .map(Json)
        .map_err(ApiError::from)
}

fn get_file_service(config: &ApplicationConfig, db: Arc<DbConn>) -> DefaultFileService {
    DefaultFileService::new(config, db)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadResponse {
    url: String,
    ignored_fields: Vec<String>,
}

enum ApiError {
    InnerErr(Error),
}

impl From<Error> for ApiError {
    fn from(inner: Error) -> Self {
        ApiError::InnerErr(inner)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::InnerErr(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
        };

        let body = Json(json!({
            "error": "API error",
            "message": error_message,
        }));

        (status, body).into_response()
    }
}
