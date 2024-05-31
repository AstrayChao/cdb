use std::fs;
use std::sync::OnceLock;
use config::Config;

pub mod logger;
pub mod database;
pub mod route;

static CFG: OnceLock<Config> = OnceLock::new();

pub fn init_config() {
    let path = fs::canonicalize("config/app.yml")
        .unwrap_or_else(|_| panic!("app.yml not found"));

    let cfg = config::Config::builder()
        .add_source(config::File::with_name(path.to_str().unwrap()))
        .build()
        .unwrap_or_else(|e| panic!("Error loading config: {}", e));

    let _ = CFG.set(cfg);
}

pub fn get() -> &'static Config {
    CFG.get().unwrap_or_else(|| panic!("Config not initialized"))
}