use std::sync::Arc;

use crate::models;
pub async fn register_user(name: String, email: String, password: String, db: models::db::Database) -> Result<models::user::User, String> {
    let mut db = db;
    if db.get_user_by_email(&email).is_some() {
        return Err("Email already registered".to_string());
    }
    let user = models::user::User::new(name, email, password);
    db.add_user(user.clone());
    Ok(user)
}