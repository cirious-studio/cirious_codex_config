//! # Cirious Codex Config
//!
//! `cirious_codex_config` is a robust configuration management framework tailored for the
//! Cirious ecosystem. It provides a highly optimized foundation for loading, parsing,
//! and validating application settings from various sources, ensuring a secure and
//! reliable setup process.
//!
//! ## Overview
//!
//! - **Flexible Loading**: Built-in support for merging configurations from multiple sources, including files (TOML, JSON, YAML) and environment variables.
//! - **Structured Deserialization**: Seamless integration with `serde` to map your settings directly into strongly-typed Rust structures.
//! - **Diagnostic Integration**: Designed to work flawlessly with the broader Cirious Codex ecosystem to provide detailed, actionable error tracking when configuration issues occur.

/// Contains abstractions for parsing configuration files in various formats
///
/// (JSON, TOML, YAML). Use the appropriate feature flags to enable them.
pub mod format;

/// Configuration builder module that handles merging and overrides.
pub mod builder;

pub(crate) mod utils;

pub use builder::ConfigBuilder;
pub use serde::{Deserialize, Serialize};
