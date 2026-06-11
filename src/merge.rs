use serde_json::Value;

/// Performs a deep merge of `source` into `target`.
/// If both are Objects, they are merged recursively.
/// Otherwise, `source` overwrites `target`.
pub fn deep_merge(target: &mut Value, source: Value) {
  match (target, source) {
    (Value::Object(target_map), Value::Object(source_map)) => {
      for (key, value) in source_map {
        deep_merge(target_map.entry(key).or_insert(Value::Null), value);
      }
    }
    // Add array merge support
    (Value::Array(target_vec), Value::Array(source_vec)) => {
      target_vec.extend(source_vec);
    }
    (target, source) => {
      *target = source;
    }
  }
}
