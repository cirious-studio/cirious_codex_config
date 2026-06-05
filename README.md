<div align="center">

# ⚙️ Cirious Codex Config

**Robust Configuration Management Framework**

[![CI](https://github.com/cirious-studio/cirious_codex_config/actions/workflows/ci.yml/badge.svg)](https://github.com/cirious-studio/cirious_codex_config/actions/workflows/ci.yml) [![Crates.io](https://img.shields.io/crates/v/cirious_codex_config.svg)](https://crates.io/crates/cirious_codex_config) [![Docs.rs](https://docs.rs/cirious_codex_config/badge.svg)](https://docs.rs/cirious_codex_config) [![Language](https://img.shields.io/badge/Language-Rust-black?logo=rust)](https://www.rust-lang.org/) [![License](https://img.shields.io/badge/License-MIT%2FApache-blue.svg)](#-license)

</div>

---

## 📖 Overview

**Cirious Codex Config** is a highly optimized foundational library designed as a complete **Configuration Management Framework**. 

It provides a rich, generic envelope around application settings, guaranteeing that configuration loading, parsing, and validation—whether from files, environment variables, or other sources—is handled securely and reliably.

Designed to be the immutable bedrock for configuration handling within the Cirious ecosystem, prioritizing maximum observability, flexible deserialization via `serde`, and a flawless developer experience.

---

## ✨ Features

- Support for multiple configuration formats (JSON, TOML, YAML).
- Environment variable overrides.
- Robust validation and error tracking using `cirious_codex_result`.
- Integrate optional feature for terminal color support with `cirious_codex_term`.

---

## 🚀 Quick Start
 
Add the following to your `Cargo.toml`:

```toml
[dependencies]
cirious_codex_config = "0.1.0"
```

And then in your code:

```rust
use cirious_codex_config::{format::ConfigFormat, ConfigBuilder, Deserialize};

#[derive(Debug, Deserialize)]
struct AppSettings {
    app_name: String,
    debug_mode: bool,
}

fn main() {
    let ron_content = r#"
        (
            app_name: "Codex Engine",
            debug_mode: true,
        )
    "#;

    // Build configuration using multiple sources (File + Environment Variables)
    let settings: AppSettings = ConfigBuilder::new()
        .add_source(ron_content, ConfigFormat::Ron)
        .unwrap()
        .value
        .add_env_prefix("APP_") // e.g. APP_DEBUG_MODE=false
        .build::<AppSettings>()
        .unwrap()
        .value;
    
    println!("Loaded config for: {}", settings.app_name);
}
```

---

## 🚧 Current Status & Roadmap

The architecture is currently being mapped out for the initial `v0.2` release. Planned features include:

- [ ] **Hierarchical Env Vars**: Map nested environment variables to structs.
- [ ] **Live Reloading**: Hot-reload configurations via file system watches.
- [ ] **Advanced Merge**: Deep merge nested configs across multiple sources.
- [ ] **CLI Integration**: Override configurations via command-line arguments.
- [ ] **Secrets Management**: Securely resolve encrypted secrets and vaults.

---

## 📜 License

Licensed under either of the following, at your option:

* **[MIT License](LICENSE-MIT)**
* **[Apache License 2.0](LICENSE-APACHE)**

---

<div align="center">
  <i>Minimalist by design. Consistent in execution.</i><br>
  <sub>Engineered by Cirious Studio</sub>
</div>
