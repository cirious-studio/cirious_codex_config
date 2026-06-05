#![allow(missing_docs)]
#![allow(clippy::unwrap_used)]

use cirious_codex_config::{format::ConfigFormat, Deserialize};

#[derive(Debug, Deserialize)]
struct AppSettings {
  app_name: String,
  debug_mode: bool,
}

fn main() {
  let ron_content = r#"
        (
            app_name: "My Awesome App",
            debug_mode: true,
        )
    "#;

  std::env::set_var("APP_DEBUG_MODE", "false");

  println!("Loading configuration...");

  let result = cirious_codex_config::ConfigBuilder::new()
    .add_source(ron_content, ConfigFormat::Ron)
    .unwrap()
    .value
    .add_env_prefix("APP_")
    .build::<AppSettings>();

  match result {
    Ok(ok_wrapper) => {
      let settings = ok_wrapper.value;
      println!("App Name: {}", settings.app_name);
      println!("Debug Mode: {}", settings.debug_mode);
    }
    Err(e) => eprintln!("Failed: {e}"),
  }
}
