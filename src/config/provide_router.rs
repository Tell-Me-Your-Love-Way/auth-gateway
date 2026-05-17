use salvo::prelude::*;

use crate::handlers;

pub fn build() -> Router {
    Router::new().push(Router::with_path("/api").hoop(handlers::set_db).hoop(max_concurrency(1)).get(handlers::health_check::handle).post(handlers::register::handle))
}