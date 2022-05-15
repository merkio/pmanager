use sea_orm::DbConn;
use domain::{Repository, Resource, User, Storage};

pub struct CONTEXT {
    db: DbConn,
    resources: Box<dyn Repository<Type=Resource>>,
    users: Box<dyn Repository<Type=User>>,
    storage: Box<dyn Storage>,
}