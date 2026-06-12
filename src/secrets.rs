use cirious_codex_result::{CodexOkRaw, Result};
use serde_json::Value;
use std::{collections::HashMap, fmt::Debug};

/// A trait for resolving secrets from a secret provider.
pub trait SecretProvider: Send + Sync + Debug {
  /// Resolves a secret from the secret provider.
  ///
  /// # Arguments
  ///
  /// * `secret_id` - The ID of the secret to resolve.
  ///
  /// # Returns
  ///
  /// Returns the resolved secret as a string.
  ///
  /// # Errors
  ///
  /// Returns an error if the secret cannot be resolved.
  fn resolve(&self, secret_id: &str) -> Result<String>;
}

/// Resolves secrets in a value.
#[must_use]
pub fn resolve_with_providers<S: ::std::hash::BuildHasher>(
  secret_url: &str,
  providers: &HashMap<String, Box<dyn SecretProvider>, S>,
) -> Option<CodexOkRaw<String>> {
  let parts: Vec<&str> = secret_url.splitn(2, "://").collect();
  if parts.len() < 2 {
    return None;
  }

  let scheme = parts[0];
  let secret_id = parts[1];

  providers
    .get(scheme)
    .and_then(|provider| provider.resolve(secret_id).ok())
}

/// Resolves secrets in a value.
pub fn resolve_secrets<S: ::std::hash::BuildHasher>(
  value: &mut Value,
  providers: &HashMap<String, Box<dyn SecretProvider>, S>,
) {
  match value {
    Value::String(s) if s.starts_with("vault://") => {
      if let Some(resolved) = resolve_with_providers(s, providers) {
        *value = Value::String(resolved.value);
      }
    }
    Value::Object(map) => {
      for v in map.values_mut() {
        resolve_secrets(v, providers);
      }
    }
    Value::Array(arr) => {
      for v in arr {
        resolve_secrets(v, providers);
      }
    }
    _ => {}
  }
}
