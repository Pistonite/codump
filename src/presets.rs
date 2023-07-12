//! Comment pattern presets for different languages

use crate::process::CommentPattern;
#[cfg(feature = "cli")]
use clap::ValueEnum;
use regex::Regex;

/// Preset values
#[derive(Debug, Clone)]
#[cfg_attr(feature = "cli", derive(ValueEnum))]
pub enum Preset {
    /// Rust style
    ///
    /// Outer comments: `///`
    /// Inner comments: `//!`
    Rust,
    /// Rust style for single line and Java/JS/TS style for multiline
    ///
    /// Outer comments: `///` and `/** ... */`
    /// Inner comments: `//!` and `/* ... */`
    RustJava,
    /// Python style
    ///
    /// Outer comments: empty line
    /// Inner comments: `###` and `""" ... """`
    Python,
}

/// Preset implementation
impl Preset {
    /// Get the patterns in the preset
    ///
    /// Returns (outer, inner)
    pub fn get_patterns(&self) -> (CommentPattern, CommentPattern) {
        match self {
            Preset::Rust => (
                CommentPattern {
                    single_line: Regex::new(r"^///").unwrap(),
                    multi_start: None,
                    multi_end: Regex::new(r"").unwrap(),
                },
                CommentPattern {
                    single_line: Regex::new(r"^//!").unwrap(),
                    multi_start: None,
                    multi_end: Regex::new(r"").unwrap(),
                },
            ),
            Preset::RustJava => (
                CommentPattern {
                    single_line: Regex::new(r"^///|^/\*\*.*\*/\s*$").unwrap(),
                    multi_start: Some(Regex::new(r"^/\*\*").unwrap()),
                    multi_end: Regex::new(r"\*/\s*$").unwrap(),
                },
                CommentPattern {
                    single_line: Regex::new(r"^//!|^/\*[^\*].*\*/\s*$").unwrap(),
                    multi_start: Some(Regex::new(r"^/\*[^\*]").unwrap()),
                    multi_end: Regex::new(r"\*/\s*$").unwrap(),
                },
            ),
            Preset::Python => (
                CommentPattern {
                    single_line: Regex::new(r"^$").unwrap(),
                    multi_start: None,
                    multi_end: Regex::new(r"").unwrap(),
                },
                CommentPattern {
                    single_line: Regex::new("^[\"']{3}.*?[\"']{3}\\s*$").unwrap(),
                    multi_start: Some(Regex::new("^[\"']{3}").unwrap()),
                    multi_end: Regex::new("[\"']{3}\\s*$").unwrap(),
                },
            ),
        }
    }
}
