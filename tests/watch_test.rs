//! Integration test for file watcher functionality.
//!
//! Validates that the watcher can be started and effectively monitors
//! file modifications in a non-blocking test environment.

use cirious_codex_config::watch_config;
use std::fs;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[test]
fn test_watch_config_triggers() -> Result<(), Box<dyn std::error::Error>> {
  let path = "test_config.toml";

  // Cleanup de segurança antes do teste
  if fs::metadata(path).is_ok() {
    fs::remove_file(path)?;
  }

  fs::File::create(path)?;

  let trigger_count = Arc::new(Mutex::new(0));
  let trigger_clone = trigger_count.clone();

  // Start watcher in a background thread
  thread::spawn(move || {
    // Ignoramos o retorno aqui pois o watcher é um processo de longa duração
    // que só termina se houver falha crítica ou fecho da app
    let _ = watch_config(path, move || {
      if let Ok(mut count) = trigger_clone.lock() {
        *count += 1;
      }
    });
  });

  // Small delay to allow watcher to initialize
  thread::sleep(Duration::from_millis(500));

  // Modify the file
  let mut file = fs::File::create(path)?;
  file.write_all(b"key = 'value'")?;

  // Small delay for event propagation
  thread::sleep(Duration::from_millis(500));

  let count = *trigger_count.lock().map_err(|_| "Mutex poisoned")?;

  // Cleanup final
  fs::remove_file(path)?;

  assert!(count > 0, "Watcher should have triggered at least once");

  Ok(())
}
