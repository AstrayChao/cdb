use tracing_appender::non_blocking::WorkerGuard;

use crate::init;
use crate::init::{database, init_config, logger};


pub async fn init() -> WorkerGuard {
    init_config();
    let guard = logger::init_logger(Some(init::get()));
    database::init_database(init::get()).await;
    guard
}


pub async fn run() {
    let port = init::get().get_int("app.port").unwrap_or(11451);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    axum::serve(listener, init::route::init()).await.unwrap();
}

