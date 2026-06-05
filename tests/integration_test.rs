#![allow(missing_docs)]
#![allow(clippy::unwrap_used)]

use cirious_codex_config::{format::ConfigFormat, Deserialize};

#[derive(Deserialize)]
struct Config {
  key: String,
}

#[test]
fn test_public_api() {
  let ron_data = r#"( key: "value" )"#;
  let parsed: Config = ConfigFormat::Ron.parse(ron_data).unwrap().value;

  assert_eq!(parsed.key, "value");
}
