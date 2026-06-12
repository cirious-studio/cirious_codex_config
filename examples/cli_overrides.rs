//! Example: Command-line overrides.
//!
//! This example demonstrates how to merge command-line arguments into the
//! configuration structure using dot-notation for nested fields.

use cirious_codex_config::ConfigBuilder;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct Database {
  url: String,
  pool_size: u64,
}

#[derive(Debug, Deserialize)]
struct AppConfig {
  database: Database,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Simulating CLI arguments (e.g., --database.pool_size=50)
  let mut args = HashMap::new();
  args.insert(
    "database.url".to_string(),
    "postgres://db.example.com:5432/prod".to_string(),
  );
  args.insert("database.pool_size".to_string(), "50".to_string());

  // Build configuration applying CLI overrides
  let config = ConfigBuilder::new().add_cli_overrides(args).build::<AppConfig>()?.value;

  println!("Database URL: {}", config.database.url);
  println!("Updated Pool Size: {}", config.database.pool_size);
  Ok(())
}
