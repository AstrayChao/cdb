use axum::Router;
use axum::routing::get;
use crate::api::user::list;

pub fn init() -> Router {
    Router::new()
        .route("/", get(|| async { "😸😸" }))
        .route("/users", get(list))
}