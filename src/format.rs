//! Configuration formatting and parsing definitions.

use cirious_codex_result::{codex_ok, CodexError, Result};
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
  pub fn parse<T: DeserializeOwned>(&self, content: &str) -> Result<T> {
    match self {
      Self::Ron => match ron::from_str::<T>(content) {
        Ok(v) => codex_ok!(v),
        Err(e) => Err(CodexError::builder(
          crate::utils::format_error_code("RON_PARSE_ERROR"),
          format!("Invalid RON format: {e}"),
        )),
      },
      Self::Json => match serde_json::from_str::<T>(content) {
        Ok(v) => codex_ok!(v),
        Err(e) => Err(CodexError::builder(
          crate::utils::format_error_code("JSON_PARSE_ERROR"),
          format!("Invalid JSON format: {e}"),
        )),
      },

      #[cfg(feature = "toml")]
      Self::Toml => match toml::from_str::<T>(content) {
        Ok(v) => codex_ok!(v),
        Err(e) => Err(CodexError::builder(
          crate::utils::format_error_code("TOML_PARSE_ERROR"),
          format!("Invalid TOML format: {e}"),
        )),
      },

      #[cfg(feature = "yaml")]
      Self::Yaml => match serde_yaml::from_str::<T>(content) {
        Ok(v) => codex_ok!(v),
        Err(e) => Err(CodexError::builder(
          crate::utils::format_error_code("YAML_PARSE_ERROR"),
          format!("Invalid YAML format: {e}"),
        )),
      },

      #[allow(unreachable_patterns)]
      _ => Err(CodexError::builder(
        crate::utils::format_error_code("FORMAT_NOT_ENABLED"),
        "Selected format is not enabled via features",
      )),
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

    let result: Result<DummyConfig> = ConfigFormat::Ron.parse(content);

    assert!(result.is_ok());
    let config = result.unwrap().value;
    assert_eq!(config.name, "Codex");
    assert_eq!(config.version, 1);
  }

  #[test]
  fn test_parse_json() {
    let content = r#"{
            "name": "Codex",
            "version": 1
        }"#;

    let result: Result<DummyConfig> = ConfigFormat::Json.parse(content);
    assert!(result.is_ok());
  }

  #[cfg(feature = "yaml")]
  #[test]
  fn test_parse_yaml() {
    let content = "
            name: Codex
            version: 1
        ";

    let result: Result<DummyConfig> = ConfigFormat::Yaml.parse(content);
    assert!(result.is_ok());
  }

  #[cfg(feature = "toml")]
  #[test]
  fn test_parse_toml() {
    let content = r#"
            name = "Codex"
            version = 1
        "#;

    let result: Result<DummyConfig> = ConfigFormat::Toml.parse(content);
    assert!(result.is_ok());
  }
}
