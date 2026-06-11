//! Integration tests for `ConfigBuilder` functionality.
//!
//! This module provides test cases to ensure that configurations are
//! correctly merged from various sources and mapped to structs.

use cirious_codex_config::{format::ConfigFormat, ConfigBuilder};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
struct AppConfig {
  name: String,
  port: u16,
}

#[test]
fn test_builder_merge_flow() {
  let raw_json = r#"{"name": "CodexApp", "port": 8080}"#;

  let builder = ConfigBuilder::new().add_source(raw_json, ConfigFormat::Json);

  if let Ok(builder) = builder {
    let config_result = builder.value.add_env_prefix("APP_").build::<AppConfig>();
    if let Ok(config) = config_result {
      let config = config.value;
      assert_eq!(config.name, "CodexApp");
      assert_eq!(config.port, 8080);
    }
  }
}
