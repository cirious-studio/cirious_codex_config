//! Example: Basic configuration loading.
//!
//! This example shows how to initialize the `ConfigBuilder`, add a JSON source,
//! and override specific values using environment variables.

use cirious_codex_config::{format::ConfigFormat, ConfigBuilder};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ServerConfig {
  host: String,
  port: u16,
}

fn main() {
  // Simulated JSON configuration file content
  let raw_json = r#"{
        "host": "127.0.0.1",
        "port": 3000
    }"#;

  // Build configuration
  // APP_PORT=8080 cargo run --example basic_config
  let builder = ConfigBuilder::new().add_source(raw_json, ConfigFormat::Json);

  if let Ok(builder) = builder {
    let config_result = builder.value.add_env_prefix("APP_").build::<ServerConfig>();
    if let Ok(config) = config_result {
      let config = config.value;
      println!("Server running at: {}:{}", config.host, config.port);
    }
  }
}
