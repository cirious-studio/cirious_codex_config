#![allow(missing_docs)]

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

  println!("Loading configuration...");

  match ConfigFormat::Ron.parse::<AppSettings>(ron_content) {
    Ok(settings) => {
      println!("Success!");
      println!("App Name: {}", settings.app_name);
      println!("Debug Mode: {}", settings.debug_mode);
    }
    Err(e) => {
      eprintln!("Failed to load config: {e}");
    }
  }
}
