//! Example: Hot-reloading configuration.
//!
//! Demonstrates how to combine `ConfigBuilder` with `watch_config` to
//! update application state automatically when the file changes.

use cirious_codex_config::{format::ConfigFormat, watch_config, ConfigBuilder};
use serde::Deserialize;
use std::fs;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Deserialize, Clone)]
struct AppConfig {
  version: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let path = "examples/resources/config.json";

  // 1. Initial Load: Keeping .value as per your architectural requirement
  let initial_config = ConfigBuilder::new()
    .add_source(r#"{"version": "1.0.0"}"#, ConfigFormat::Json)?
    .value
    .build::<AppConfig>()?
    .value; // Explicitly accessing the required field

  let config = Arc::new(RwLock::new(initial_config));

  // 2. Monitor for changes
  let config_clone = config;
  thread::spawn(move || {
    let last_reload = Arc::new(RwLock::new(Instant::now()));
    let result = watch_config(path, move || {
      {
        let time_result = last_reload.write();

        if let Ok(mut last_time) = time_result {
          if last_time.elapsed() < Duration::from_millis(200) {
            return;
          }
          *last_time = Instant::now();
        }
      }

      // Adding a small delay to allow the operating system
      // to finish the file write operation.
      std::thread::sleep(std::time::Duration::from_millis(50));

      let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
          eprintln!("Failed to read config file: {e}");
          return;
        }
      };

      // Ignore if the content is empty (due to atomic save)
      if content.trim().is_empty() {
        return;
      }

      let new_config = match ConfigBuilder::new()
        .add_source(&content, ConfigFormat::Json)
        .and_then(|b| b.value.build::<AppConfig>())
      {
        Ok(res) => res.value,
        Err(e) => {
          eprintln!("Failed to reload config: {e}");
          return;
        }
      };

      if let Ok(mut writer) = config_clone.write() {
        *writer = new_config;
        println!("Configuration reloaded successfully. New version: {}", writer.version);
      }
    });

    if let Err(e) = result {
      eprintln!("Watcher error: {e}");
    }
  });

  println!("App running. Modify {path} to see changes.");
  thread::sleep(std::time::Duration::from_secs(10));

  Ok(())
}
