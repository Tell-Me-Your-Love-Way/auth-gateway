use std::{collections::HashMap, sync::{Arc, Mutex}};

#[derive(Clone, Debug)]
pub struct Database {
    user_by_id: Arc<Mutex<HashMap<String, super::user::User>>>,
    user_by_email: Arc<Mutex<HashMap<String, super::user::User>>>,
}
impl Database {
    pub fn new() -> Self {
        let user: HashMap<String, super::user::User> = HashMap::new();
        let user: Arc<Mutex<HashMap<String, super::user::User>>> = Arc::new(Mutex::new(user));
        Database {
            user_by_id: user.clone(),
            user_by_email: user.clone(),
        }
    }
    pub fn add_user(&mut self, user: super::user::User) {
        self.user_by_id.lock().unwrap().insert(user.id().to_string(), user.clone());
        self.user_by_email.lock().unwrap().insert(user.email().to_owned().clone(), user);
    }
    pub fn get_user_by_id(&self, id: &str) -> Option<super::user::User> {
        self.user_by_id.lock().unwrap().get(id).cloned()
    }
    pub fn get_user_by_email(&self, email: &str) -> Option<super::user::User> {
        self.user_by_email.lock().unwrap().get(email).cloned()
    }
}