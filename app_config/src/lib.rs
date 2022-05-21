use config::{Config, Environment, File};
use serde::Deserialize;
use std::env;

const CONFIG_FILE_PATH: &str = "./Default.toml";
const CONFIG_FILE_PREFIX: &str = "./";

#[derive(Debug, Deserialize, Clone)]
pub struct App {
    pub version: String,
    pub host: String,
    pub port: String,
    pub debug: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DbConnection {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AwsConfig {
    pub secret_access_key: String,
    pub access_key_id: String,
    pub region: String,
    pub endpoint: String,
    pub bucket: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ApplicationConfig {
    pub app: App,
    pub db: DbConnection,
    pub aws: AwsConfig,
}

impl Default for ApplicationConfig {
    fn default() -> Self {
        dotenv::dotenv().ok();

        let env = env::var("ENV").unwrap_or_else(|_| "Development".into());
        Config::builder()
            .add_source(File::with_name(CONFIG_FILE_PATH))
            .add_source(File::with_name(&format!("{}{}", CONFIG_FILE_PREFIX, env)))
            .add_source(
                Environment::with_prefix("P")
                    .try_parsing(true)
                    .separator("_"),
            )
            .build()
            .unwrap()
            .try_deserialize()
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_db_connection_config() {
        env::set_var("P_DB_URL", String::from("localhost"));

        let config = ApplicationConfig::default();
        assert_eq!(config.db.url, String::from("localhost"));
    }

    #[test]
    fn test_app_config() {
        env::set_var("P_APP_DEBUG", String::from("true"));
        env::set_var("P_APP_PORT", String::from("8111"));

        let config = ApplicationConfig::default();
        assert!(config.app.debug);
        assert_eq!(config.app.port, String::from("8111"));
    }

    #[test]
    fn test_aws_config() {
        env::set_var("P_AWS_BUCKET", String::from("files"));
        env::set_var("P_AWS_REGION", String::from("eu-central-1"));

        let config = ApplicationConfig::default();
        assert!(config.app.debug);
        assert_eq!(config.aws.bucket, String::from("files"));
        assert_eq!(config.aws.region, String::from("eu-central-1"));
    }
}
