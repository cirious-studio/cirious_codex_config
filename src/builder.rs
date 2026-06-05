use crate::format::ConfigFormat;
use cirious_codex_result::{codex_ok, CodexError, Result};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::env;

/// A robust builder for constructing, merging, and resolving configurations.
///
/// `ConfigBuilder` aggregates settings from multiple discrete sources (such as
/// structured files and environment variables), seamlessly merging them into a
/// unified internal representation before deserializing into a strongly-typed
/// structure.
#[derive(Debug, Default)]
pub struct ConfigBuilder {
  internal_map: Value,
}

impl ConfigBuilder {
  /// Creates a new, empty `ConfigBuilder`.
  #[must_use]
  pub fn new() -> Self {
    Self {
      internal_map: Value::Object(serde_json::Map::new()),
    }
  }

  /// Incorporates a structured configuration source into the builder.
  ///
  /// This method parses the provided raw string content according to the specified
  /// `ConfigFormat` and recursively merges the resulting data into the current
  /// configuration state.
  ///
  /// # Errors
  /// Returns an error if the content cannot be parsed by the specified format.
  pub fn add_source(mut self, content: &str, format: ConfigFormat) -> Result<Self> {
    let parsed = format.parse::<Value>(content).map_err(|e| {
      e.with_suggestion(crate::utils::format_suggestion(
        "Check the configuration file syntax for invalid formatting.",
      ))
      .with_meta("format", format!("{format:?}"))
    })?;

    self.merge_value(parsed.value);

    codex_ok!(self)
  }

  /// Incorporates environment variables into the configuration state.
  ///
  /// This method iterates through the current process's environment variables,
  /// filtering for those that start with the provided `prefix`. Matching variables
  /// have their prefix stripped and are converted to lowercase to align with standard
  /// configuration property casing.
  #[must_use]
  pub fn add_env_prefix(mut self, prefix: &str) -> Self {
    for (key, val) in env::vars() {
      if let Some(stripped) = key.strip_prefix(prefix) {
        // Strips the prefix and converts the key to lowercase (e.g., APP_PORT -> port).
        let clean_key = stripped.to_lowercase();

        // Attempts to parse the value as a number or a boolean; falls back to a string if parsing fails.
        let parsed_val = val
          .parse::<i64>()
          .map(|num| Value::Number(num.into()))
          .or_else(|_| val.parse::<bool>().map(Value::Bool))
          .unwrap_or(Value::String(val));

        // Injects the parsed value into the intermediate configuration map.
        if let Value::Object(ref mut map) = self.internal_map {
          map.insert(clean_key, parsed_val);
        }
      }
    }
    self
  }

  /// Finalizes the build process and deserializes the aggregated configuration.
  ///
  /// This method consumes the `ConfigBuilder`, attempting to map the deeply merged
  /// internal JSON representation into the designated strongly-typed structure `T`.
  ///
  /// # Errors
  /// Returns an error if the merged configuration cannot be mapped to the target type `T`.
  pub fn build<T: DeserializeOwned>(self) -> Result<T> {
    let result = serde_json::from_value(self.internal_map).map_err(|e| {
      CodexError::builder(
        "CONFIG_BUILD_ERROR",
        format!("Failed to map merged configurations to target struct: {e}"),
      )
      .with_suggestion("Ensure your struct properties match the loaded data and no required fields are missing.")
    })?;

    codex_ok!(result)
  }

  /// Recursively merges a parsed `serde_json::Value` into the internal configuration map.
  fn merge_value(&mut self, other: Value) {
    merge(&mut self.internal_map, other);
  }
}

/// A recursive utility function to merge two `serde_json::Value` instances.
fn merge(a: &mut Value, b: Value) {
  match (a, b) {
    (&mut Value::Object(ref mut a_map), Value::Object(b_map)) => {
      for (k, v) in b_map {
        merge(a_map.entry(k).or_insert(Value::Null), v);
      }
    }
    (a_val, b_val) => *a_val = b_val,
  }
}
