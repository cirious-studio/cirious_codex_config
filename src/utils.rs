#[cfg(feature = "term")]
use cirious_codex_term::StyleExt;

/// Helper to conditionally colorize error components based on the `term` feature.
pub fn format_error_code(code: &str) -> String {
  #[cfg(feature = "term")]
  {
    code.red().bold().to_string()
  }
  #[cfg(not(feature = "term"))]
  {
    code.to_string()
  }
}

pub fn format_suggestion(sug: &str) -> String {
  #[cfg(feature = "term")]
  {
    sug.yellow().to_string()
  }
  #[cfg(not(feature = "term"))]
  {
    sug.to_string()
  }
}
