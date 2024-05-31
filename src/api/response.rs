use serde::{Deserialize, Serialize};
use crate::model::user::Model;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserResponse {
    pub id: i64,
    pub username: String,
    pub permission: i16,
    pub created_at: i64,
    pub updated_at: i64,
}


impl From<&Model> for UserResponse {
    fn from(u: &Model) -> Self {
        Self {
            id: u.id,
            username: u.username.clone(),
            permission: u.permission,
            created_at: u.created_at,
            updated_at: u.updated_at
        }
    }
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserListResponse {
    pub users: Vec<UserResponse>,
}