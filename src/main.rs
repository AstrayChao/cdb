use crate::bootstrap::run;

mod service;
mod model;
mod api;
mod middleware;
mod util;
mod common;
mod bootstrap;
mod init;

#[tokio::main]
async fn main() {
    let _guard = bootstrap::init().await;
    run().await;
}
