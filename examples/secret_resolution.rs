//! Example: Secret Resolution.
//!
//! Demonstrates how to register a secret provider (e.g., `HashiCorp` Vault)
//! and automatically resolve placeholders within the configuration during build.

use cirious_codex_config::{ConfigBuilder, SecretProvider};
use cirious_codex_result::codex_ok;
use serde::Deserialize;

#[derive(Debug)]
struct VaultProvider;

impl SecretProvider for VaultProvider {
  fn resolve(&self, secret_id: &str) -> cirious_codex_result::Result<String> {
    // In a real scenario, this would perform an HTTP request to the Vault API.
    codex_ok!(format!("resolved_value_for_{}", secret_id))
  }
}

#[derive(Debug, Deserialize)]
struct AppConfig {
  api_key: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  std::env::set_var("APP_API_KEY", "vault://my-secret-key");

  let builder = ConfigBuilder::new()
    .register_provider("vault", Box::new(VaultProvider))
    .add_env_nested("APP_", "__");

  let config = builder.build::<AppConfig>()?;

  println!("Resolved API Key: {}", config.value.api_key);
  Ok(())
}
