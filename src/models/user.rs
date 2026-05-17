use argon2::{password_hash::{SaltString, rand_core::OsRng}, *};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    id: String,
    name: String,
    email: String,
    password_hash: String,
    created_at: String,
}
impl User {
    pub fn new(name: impl Into<String>, email: impl Into<String>, password: impl Into<String>) -> Self {
        let id = uuid::Uuid::now_v7().to_string();
        let name = name.into();
        let email = email.into();
        let password_hash = Self::gen_password_hash(password.into());
        let created_at = time::OffsetDateTime::now_utc().to_string();
        User {
            id,
            name,
            email,
            password_hash,
            created_at,
        }
    }
    fn gen_password_hash(password: String) -> String {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        argon2.hash_password(password.as_bytes(), &salt).expect("Failed to hash password").to_string()
    }
    pub fn id(&self) -> uuid::Uuid {
        Uuid::parse_str(&self.id).expect("Invalid ID")
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn email(&self) -> &str {
        &self.email
    }
    pub fn password_hash(&self) -> &str {
        &self.password_hash
    }
    pub fn created_at(&self) -> time::OffsetDateTime {
        time::OffsetDateTime::parse(&self.created_at, &time::format_description::well_known::Rfc3339).expect("Invalid created_at")
    }
}