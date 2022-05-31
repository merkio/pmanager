use application::FileService;
use axum::{
    extract::{Multipart, Path},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use log::info;
use serde::{Deserialize, Serialize};

pub fn router(_file_service: Box<dyn FileService>) -> Router {
    Router::new()
        .route("/download/:key", get(download))
        .route("/upload", post(upload))
}

async fn upload(mut multipart: Multipart) -> impl IntoResponse {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap_or("no name").to_owned();
        let file_name = field.file_name().unwrap_or("no file name").to_owned();
        let content_type = field.content_type().unwrap_or("no content type").to_owned();
        let data = field.bytes().await.unwrap_or_default();

        info!(
            "Part: \n Name: {}, File Name: {}, Content Type: {}, Data: {:?}",
            name, file_name, content_type, data
        );
    }
    Json(UploadResponse {
        key: "Uploaded key".to_owned(),
    })
}

async fn download(Path(key): Path<String>) -> impl IntoResponse {
    info!("Download file with key: {}", key);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadResponse {
    pub key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DownloadResponse {
    pub key: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use async_trait::async_trait;
    use axum::http::StatusCode;
    use axum_test_helper::TestClient;
    use domain::FileObject;
    use mockall::predicate::*;
    use mockall::*;
    use reqwest::multipart::Form;
    use test_log::test;

    mock! {

        pub DefaultFileService {
            fn upload(&self) -> String {}

            fn download(&self) -> FileObject {}
        }
    }

    #[async_trait]
    impl FileService for MockDefaultFileService {
        async fn upload(&self, _object: FileObject) -> Result<String> {
            Ok(self.upload())
        }

        async fn download(&self, _key: &str) -> Result<FileObject> {
            Ok(self.download())
        }
    }

    #[test(tokio::test)]
    async fn upload() {
        let _ = env_logger::builder().is_test(true).try_init();
        // const BYTES: &[u8] = "<!doctype html><title>🦀</title>".as_bytes();
        // const FILE_NAME: &str = "index.html";
        // const CONTENT_TYPE: &str = "text/html; charset=utf-8";

        let mock = MockDefaultFileService::new();
        // mock.expect_upload().return_const("Key".to_owned());
        let app = router(Box::new(mock));

        // let form = Form::new().text("key", "Key");
        // .text("tags", "test,upload")
        // .text("metadata", "{}");

        // let file = Part::bytes(BYTES)
        //     .file_name(FILE_NAME)
        //     .mime_str(CONTENT_TYPE)
        //     .unwrap();

        // let form = Form::new().part(
        //     "file",
        //     Part::bytes(BYTES)
        //         .file_name(FILE_NAME)
        //         .mime_str(CONTENT_TYPE)
        //         .unwrap(),
        // );
        let form = Form::new();

        let client = TestClient::new(app);
        let response = client.post("/upload").multipart(form).send().await;

        let status = response.status();
        info!("RESPONSE: {}", response.text().await);
        assert_eq!(status, StatusCode::OK);
    }

    #[test(tokio::test)]
    async fn download() {
        let _ = env_logger::builder().is_test(true).try_init();

        let mut mock = MockDefaultFileService::new();
        mock.expect_upload().return_const("Key".to_owned());
        let app = router(Box::new(mock));

        let client = TestClient::new(app);
        let response = client.get("/download/Key").send().await;

        assert_eq!(response.status(), StatusCode::OK);
    }
}
