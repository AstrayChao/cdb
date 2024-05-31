use log::error;
use sea_orm::{ EntityTrait};
use crate::api::response::{UserListResponse, UserResponse};
use crate::init::database;
use crate::model::prelude::User;

pub async fn get_all_user() -> Option<UserListResponse> {
    match User::find()
        .all(database::conn()).await
    {
        Ok(u) => Some(UserListResponse {users: u.iter().map(UserResponse::from).collect()}),
        Err(e) => {
            error!("数据库异常:{}", e);
            Some(UserListResponse{users: vec![]}) }
    }
}