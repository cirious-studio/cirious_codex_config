use crate::format::ConfigFormat;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::env;

/// Builder to construct and merge configurations from multiple sources.
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
  /// Adds a configuration source (file content) to the builder.
  ///
  /// # Errors
  /// Returns an error if the content cannot be parsed by the specified format.
  pub fn add_source(mut self, content: &str, format: ConfigFormat) -> Result<Self, String> {
    // Aproveitamos que o serde_json::Value implementa Deserialize!
    let parsed: Value = format.parse(content)?;
    self.merge_value(parsed);
    Ok(self)
  }

  /// Overrides current settings with environment variables that match a specific prefix.
  #[must_use]
  pub fn add_env_prefix(mut self, prefix: &str) -> Self {
    for (key, val) in env::vars() {
      if let Some(stripped) = key.strip_prefix(prefix) {
        // Remove o prefixo e deixa em minúsculo (Ex: APP_PORT -> port)
        let clean_key = stripped.to_lowercase();

        // Tenta converter para número ou booleano, se não der, fica como string
        let parsed_val = val
          .parse::<i64>()
          .map(|num| Value::Number(num.into()))
          .or_else(|_| val.parse::<bool>().map(Value::Bool))
          .unwrap_or(Value::String(val));

        // Injeta no nosso dicionário intermediário
        if let Value::Object(ref mut map) = self.internal_map {
          map.insert(clean_key, parsed_val);
        }
      }
    }
    self
  }

  /// Builds the final configuration structure.
  ///
  /// # Errors
  /// Returns an error if the merged configuration cannot be mapped to the target type `T`.
  pub fn build<T: DeserializeOwned>(self) -> Result<T, String> {
    serde_json::from_value(self.internal_map).map_err(|e| format!("Build error: {e}"))
  }

  /// Helper para mesclar os dados JSON
  fn merge_value(&mut self, other: Value) {
    merge(&mut self.internal_map, other);
  }
}

/// Função recursiva simples para mesclar dois `serde_json::Value`
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
