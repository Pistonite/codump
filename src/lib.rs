//! # codump
//!
//! A straightforward tool for dumping code/comments from source files.

use std::io;
use std::fs;

#[cfg(feature = "cli")]
use clap::ValueEnum;
use process::FindComponentResult;
use process::find_component;
use process::parse_component;
use process::{CommentPattern, Component};
use regex::Regex;

pub mod process;
pub mod presets;

/// Configuration
#[derive(Debug, Clone)]
pub struct Config {
    /// Pattern for matching outer comments
    pub outer_comments: CommentPattern,
    /// Pattern for matching inner comments
    pub inner_comments: CommentPattern,
    /// Pattern of lines to ignore
    pub ignore_lines: Vec<Regex>,
    /// If context should include comments
    pub context_include_comments: bool,
}

/// Output format
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "cli", derive(ValueEnum))]
pub enum Format {
    /// Comments + abbreviated code
    #[default]
    Summary,

    /// Comment only format
    Comment,

    /// Comment + all code
    Detail,
}

/// Search for a component in a file
pub fn search_file(file_path: &str, search_path: &[String], config: &Config) -> io::Result<FindComponentResult> {
    let component = parse_file(file_path, config)?;

    Ok(find_component(&component, search_path, config))
}

/// Parse a file into a component
pub fn parse_file(path: &str, config: &Config) -> io::Result<Component> {
    let file_lines = fs::read_to_string(path)?.lines().map(|s| s.to_string()).collect();

    Ok(parse_component(
        vec![],
        file_lines, 
        0,
        config,
    ))
}

