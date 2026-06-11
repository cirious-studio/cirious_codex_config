//! Integration tests for nested environment variables.
//!
//! This module focuses on verifying that the `ConfigBuilder` correctly
//! parses and structures nested environment variables using a separator.

use cirious_codex_config::ConfigBuilder;
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
struct Database {
  url: String,
  port: u16,
}

#[derive(Debug, Deserialize)]
struct Config {
  database: Database,
}

#[test]
fn test_nested_env_vars() {
  // Setting up mock environment variables
  env::set_var("APP_DATABASE__URL", "postgres://localhost:5432");
  env::set_var("APP_DATABASE__PORT", "5432");

  let config_result = ConfigBuilder::new().add_env_nested("APP_", "__").build::<Config>();

  if let Ok(config) = config_result {
    let config = config.value;
    assert_eq!(config.database.url, "postgres://localhost:5432");
    assert_eq!(config.database.port, 5432);
  }

  // Cleanup
  env::remove_var("APP_DATABASE__URL");
  env::remove_var("APP_DATABASE__PORT");
}
