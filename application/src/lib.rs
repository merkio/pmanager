mod files;
pub use files::*;

use app_config::ApplicationConfig;
use domain::{Repository, Resource, Storage, User};
use lazy_static::lazy_static;
use once_cell::sync::OnceCell;
use remote::DefaultStorage;
use repository::{ResourceRepository, UserRepository};
use sea_orm::Database;
use std::sync::Arc;

static CURRENT_CONTEXT: OnceCell<ApplicationContext> = OnceCell::new();

lazy_static! {
    pub static ref CONTEXT: &'static ApplicationContext =
        CURRENT_CONTEXT.get_or_init(ApplicationContext::default);
}

pub struct ApplicationContext {
    pub config: Arc<ApplicationConfig>,
    pub resources: Arc<Box<dyn Repository<Type = Resource> + Send + Sync>>,
    pub users: Arc<Box<dyn Repository<Type = User> + Send + Sync>>,
    pub storage: Arc<Box<dyn Storage + Send + Sync>>,
}

impl ApplicationContext {
    pub fn init(
        config: ApplicationConfig,
        resources: Box<dyn Repository<Type = Resource> + Send + Sync>,
        users: Box<dyn Repository<Type = User> + Send + Sync>,
        storage: Box<dyn Storage + Send + Sync>,
    ) -> Self {
        ApplicationContext {
            config: Arc::new(config),
            resources: Arc::new(resources),
            users: Arc::new(users),
            storage: Arc::new(storage),
        }
    }
}

impl Default for ApplicationContext {
    fn default() -> Self {
        let config = ApplicationConfig::default();
        let r_db = futures::executor::block_on(Database::connect(config.db.url.clone())).unwrap();
        let u_db = futures::executor::block_on(Database::connect(config.db.url.clone())).unwrap();
        let aws_config = config.aws.clone();

        ApplicationContext {
            config: Arc::new(config),
            resources: Arc::new(Box::new(ResourceRepository::new(r_db))),
            users: Arc::new(Box::new(UserRepository::new(u_db))),
            storage: Arc::new(Box::new(DefaultStorage::from_config(aws_config))),
        }
    }
}
