//! Integration tests for Command-Line interface configuration overrides.

use cirious_codex_config::ConfigBuilder;
use std::collections::HashMap;

#[test]
fn test_add_cli_overrides_nesting() -> Result<(), Box<dyn std::error::Error>> {
  let mut overrides = HashMap::new();
  overrides.insert("server.port".to_string(), "8080".to_string());
  overrides.insert("server.host".to_string(), "localhost".to_string());

  // We build into Value to verify the raw nested structure
  let config = ConfigBuilder::new()
    .add_cli_overrides(overrides)
    .build::<serde_json::Value>()?
    .value;

  // Validate the structure: { "server": { "port": 8080, "host": "localhost" } }
  let server = config.get("server").ok_or("Missing 'server' key")?;
  let port = server.get("port").ok_or("Missing 'port' key")?;
  let host = server.get("host").ok_or("Missing 'host' key")?;

  assert_eq!(port, 8080);
  assert_eq!(host, "localhost");

  Ok(())
}
