//! Configuration formatting and parsing definitions.

use serde::de::DeserializeOwned;
/// Represents the supported configuration formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigFormat {
  /// RON format (Rusty Object Notation), the default standard.
  Ron,
  /// JSON format (requires the `json` feature).
  /// JSON format (now standard since `serde_json` is a core dependency).
  Json,
  /// TOML format (requires the `toml` feature).
  #[cfg(feature = "toml")]
  Toml,
  /// YAML format (requires the `yaml` feature).
  #[cfg(feature = "yaml")]
  Yaml,
}

impl ConfigFormat {
  /// Parses a string into the desired target type `T` based on the format.
  ///
  /// # Errors
  ///
  /// Returns a `String` containing the parsing error if the provided content
  /// is not valid for the selected configuration format, or if the format feature
  /// is not enabled.
  pub fn parse<T: DeserializeOwned>(&self, content: &str) -> Result<T, String> {
    match self {
      Self::Ron => ron::from_str(content).map_err(|e| format!("RON parsing error: {e}")),
      Self::Json => serde_json::from_str(content).map_err(|e| format!("JSON parsing error: {e}")),

      #[cfg(feature = "toml")]
      Self::Toml => toml::from_str(content).map_err(|e| format!("TOML parsing error: {e}")),

      #[cfg(feature = "yaml")]
      Self::Yaml => serde_yaml::from_str(content).map_err(|e| format!("YAML parsing error: {e}")),

      #[allow(unreachable_patterns)]
      _ => Err("Selected format is not enabled via features".to_string()),
    }
  }
}

#[cfg(test)]
mod tests {
  #![allow(clippy::unwrap_used)]

  use super::*;
  use serde::Deserialize;

  #[derive(Debug, Deserialize, PartialEq)]
  struct DummyConfig {
    name: String,
    version: u8,
  }

  #[test]
  fn test_parse_ron() {
    let content = r#"(
            name: "Codex",
            version: 1,
        )"#;

    let result: Result<DummyConfig, String> = ConfigFormat::Ron.parse(content);

    assert!(result.is_ok());
    let config = result.unwrap();
    assert_eq!(config.name, "Codex");
    assert_eq!(config.version, 1);
  }

  #[test]
  fn test_parse_json() {
    let content = r#"{
            "name": "Codex",
            "version": 1
        }"#;

    let result: Result<DummyConfig, String> = ConfigFormat::Json.parse(content);
    assert!(result.is_ok());
  }

  #[cfg(feature = "yaml")]
  #[test]
  fn test_parse_yaml() {
    let content = "
            name: Codex
            version: 1
        ";

    let result: Result<DummyConfig, String> = ConfigFormat::Yaml.parse(content);
    assert!(result.is_ok());
  }

  #[cfg(feature = "toml")]
  #[test]
  fn test_parse_toml() {
    let content = r#"
            name = "Codex"
            version = 1
        "#;

    let result: Result<DummyConfig, String> = ConfigFormat::Toml.parse(content);
    assert!(result.is_ok());
  }
}
