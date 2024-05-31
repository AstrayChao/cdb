use crate::api::response::{UserListResponse};
use crate::common::result::{ApiResponse, ApiResult};
use crate::service;

pub async fn list() -> ApiResult<UserListResponse> {
    Ok(ApiResponse::ok(service::user::get_all_user().await))
}