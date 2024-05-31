use std::sync::OnceLock;
use std::time::Duration;
use config::Config;
use log::info;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

static DB: OnceLock<DatabaseConnection> = OnceLock::new();

pub async fn init_database(cfg: &Config) {
    let db = cfg.get_table("db").expect("database config is required");
    for db_name in db.keys() {
        match db_name.as_str() {
            "postgres" => {
                let url = cfg.get_string("db.postgres.url").expect("database url is required");
                info!("database url:{}", url);
                let mut options = ConnectOptions::new(url);
                options.min_connections(cfg.get_int("db.postgres.min_connections").unwrap_or(5) as u32)
                    .max_connections(cfg.get_int("db.postgres.max_connections").unwrap_or(20) as u32)
                    .connect_timeout(Duration::from_secs(cfg.get_int("db.postgres.connect_timeout").unwrap_or(10) as u64))
                    .idle_timeout(Duration::from_secs(cfg.get_int("db.postgres.idle_timeout").unwrap_or(180) as u64))
                    .max_lifetime(Duration::from_secs(cfg.get_int("db.postgres.max_lifetime").unwrap_or(600) as u64))
                    .sqlx_logging(cfg.get_bool("app.debug").unwrap_or(false));
                info!("database options:{:?}", options);
                let connection = Database::connect(options)
                    .await
                    .unwrap_or_else(|e| panic!("Failed to connect to database: {}", e));
                let _ = connection.ping()
                    .await
                    .is_err_and(|e| panic!("Failed to ping database: {}", e));
                let _ = DB.set(connection);
            }
            "redis" => {},
            _ => {
                unimplemented!("database {} is not supported", db_name)
            }
        }
    }

}

pub fn conn() -> &'static DatabaseConnection {
    DB.get().unwrap_or_else(|| panic!("Database not initialized"))
}