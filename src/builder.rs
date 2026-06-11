use crate::{deep_merge, format::ConfigFormat};
use cirious_codex_result::{codex_ok, CodexError, Result};
use serde::de::DeserializeOwned;
use serde_json::{Map, Value};
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

  /// Incorporates nested environment variables into the configuration state.
  ///
  /// This method iterates through the current process's environment variables,
  /// filtering for those that start with the provided `prefix`. It then splits
  /// the remaining key by the specified `separator` to construct a nested
  /// configuration structure.
  ///
  /// # Arguments
  ///
  /// * `prefix` - The prefix to filter relevant environment variables (e.g., "APP_").
  /// * `separator` - The delimiter used to define nesting levels (e.g., "__").
  ///
  /// # Example
  ///
  /// If you have `APP_DATABASE__PORT=5432`, calling this with `("APP_", "__")`
  /// will map it to: `{ "database": { "port": 5432 } }`.
  ///
  /// # Note
  ///
  /// Environment variables that cannot be parsed as numbers or booleans are
  /// stored as strings.
  ///
  /// # Panics
  ///
  /// Panics if `internal_map` is not an object, which should never happen
  /// under normal circumstances.
  #[must_use]
  pub fn add_env_nested(mut self, prefix: &str, separator: &str) -> Self {
    for (key, val) in env::vars() {
      if let Some(stripped) = key.strip_prefix(prefix) {
        let keys: Vec<String> = stripped.split(separator).map(str::to_lowercase).collect();
        if keys.is_empty() {
          continue;
        }

        let parsed_val = val
          .parse::<i64>()
          .map(|n| Value::Number(n.into()))
          .or_else(|_| val.parse::<bool>().map(Value::Bool))
          .unwrap_or(Value::String(val));

        let mut node = &mut self.internal_map;

        // Navega até o penúltimo nível
        for k in keys.iter().take(keys.len() - 1) {
          // Força a estrutura para Object se não for
          if !node.is_object() {
            *node = Value::Object(Map::new());
          }

          // Extraímos o map mutável, e garantimos que o borrow termina
          // antes da próxima iteração através do scope block
          node = {
            if let Some(map) = node.as_object_mut() {
              map.entry(k).or_insert_with(|| Value::Object(Map::new()))
            } else {
              return self;
            }
          };
        }

        // Aplica no último nível
        if !node.is_object() {
          *node = Value::Object(Map::new());
        }
        if let Some(map) = node.as_object_mut() {
          if let Some(last_key) = keys.last() {
            let target = map.entry(last_key).or_insert(Value::Null);
            deep_merge(target, parsed_val);
          }
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
    deep_merge(&mut self.internal_map, other);
  }
}
