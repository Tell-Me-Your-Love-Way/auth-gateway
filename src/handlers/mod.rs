pub mod health_check;
pub mod register;
use std::sync::Arc;

use salvo::prelude::*;

use crate::models;
#[handler]
pub async fn set_db(depot: &mut Depot) {
    depot.insert("db", Arc::new(models::db::Database::new()));
}