use sea_orm::DbConn;
use domain::{Repository, Resource, User};

pub struct CONTEXT {
    db: DbConn,
    resources: Box<dyn Repository<Type=Resource>>,
    users: Box<dyn Repository<Type=User>>,
}