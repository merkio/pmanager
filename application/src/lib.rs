use domain::{Repository, Resource, Storage, User};
use sea_orm::DbConn;

pub struct CONTEXT {
    db: DbConn,
    resources: Box<dyn Repository<Type = Resource>>,
    users: Box<dyn Repository<Type = User>>,
    storage: Box<dyn Storage>,
}
