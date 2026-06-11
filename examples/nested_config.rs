//! Example: Nested configuration loading.
//!
//! This example demonstrates how to map flat environment variables into
//! a deeply nested Rust structure using the `add_env_nested` method.

use cirious_codex_config::ConfigBuilder;
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
struct Database {
  url: String,
  pool_size: u64,
}

#[derive(Debug, Deserialize)]
struct AppConfig {
  database: Database,
}

fn main() {
  // Simulate setting nested environment variables
  env::set_var("APP_DATABASE__URL", "postgres://db.example.com:5432/prod");
  env::set_var("APP_DATABASE__POOL_SIZE", "20");

  // Build configuration using the nested approach
  let config = ConfigBuilder::new().add_env_nested("APP_", "__").build::<AppConfig>();

  if let Ok(ok) = config {
    let config = ok.value;
    println!("Database URL: {}", config.database.url);
    println!("Pool Size: {}", config.database.pool_size);
  }
}
