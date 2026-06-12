//! Integration tests for secret resolution mechanism.

use cirious_codex_config::{ConfigBuilder, SecretProvider};
use cirious_codex_result::codex_ok;

#[derive(Debug)]
struct MockProvider;

impl SecretProvider for MockProvider {
  fn resolve(&self, _id: &str) -> cirious_codex_result::Result<String> {
    codex_ok!("secret_data".to_string())
  }
}

#[test]
fn test_secret_resolution_in_map() -> Result<(), Box<dyn std::error::Error>> {
  let builder = ConfigBuilder::new().register_provider("mock", Box::new(MockProvider));

  // Directly inject a value containing a secret placeholder
  // In reality, this would come from a loaded file or env var
  let overrides = std::collections::HashMap::from([("database.password".to_string(), "mock://db_pass".to_string())]);

  let config = builder.add_cli_overrides(overrides).build::<serde_json::Value>()?;

  let pass = config.value.get("database").and_then(|db| db.get("password"));

  if let Some(value) = pass {
    assert_eq!(value, "secret_data");
  }

  Ok(())
}
