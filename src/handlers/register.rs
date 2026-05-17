use std::sync::Arc;

use salvo::prelude::*;
use serde::Deserialize;
use tracing::info;
use crate::{models, service};
#[derive(Debug, Deserialize)]
struct RegisterRequest {
    name: String,
    email: String,
    password: String,
}
#[handler]
pub async fn handle(req: &mut Request, res: &mut Response, depot: &mut Depot) {
    let user: RegisterRequest = req.parse_body::<RegisterRequest>().await.unwrap();
    let user = models::user::User::new(user.name, user.email, user.password);
    info!("Registering user: {:?}", user);
    let db = depot.get::<Arc<models::db::Database>>("db").unwrap().as_ref().clone();
    match service::register::register_user(user.name().to_string(), user.email().to_string(), user.password_hash().to_string(), db).await {
        Ok(user) => {
            res.render(Json(user));
        },       
        Err(_) => {
            res.render(StatusCode::BAD_REQUEST);
        }
    }
}