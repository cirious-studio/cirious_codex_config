//! Live reloading utility for Cirious Codex Config.
//!
//! Provides mechanisms to watch configuration files for changes and trigger
//! automatic reloads using the ConfigBuilder.

use notify::{RecursiveMode, Result, Watcher};
use std::path::Path;
use std::sync::mpsc::channel;

/// Sets up a file watcher that triggers a closure upon file changes.
///
/// This method utilizes the `notify` crate to monitor the specified path for
/// modifications. When a change event is detected, the provided `on_change`
/// closure is executed.
///
/// # Arguments
///
/// * `path` - The path to the configuration file or directory to be watched.
/// * `on_change` - A closure to be executed whenever the file is modified.
///
/// # Errors
///
/// Returns a `notify::Result` if:
/// * The underlying file system watcher fails to initialize.
/// * The provided `path` is invalid or inaccessible.
/// * The watcher fails to attach to the specified path.
///
/// # Example
///
/// ```rust,no_run
/// use cirious_codex_config::watch_config;
///
/// watch_config("config.toml", || {
///   println!("Config updated! Reloading...");
/// }).expect("Watcher failed to start");
/// ```
pub fn watch_config<F>(path: &str, on_change: F) -> Result<()>
where
  F: Fn() + Send + 'static,
{
  let (_tx, rx) = channel::<()>();

  let mut watcher = notify::recommended_watcher(move |res: Result<notify::Event>| {
    if let Ok(event) = res {
      if event.kind.is_modify() {
        on_change();
      }
    }
  })?;

  watcher.watch(Path::new(path), RecursiveMode::NonRecursive)?;

  // Keep the loop running to listen for events
  loop {
    if rx.recv().is_err() {
      break;
    }
  }

  Ok(())
}
