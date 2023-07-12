//! # codump
//!
//! A straightforward tool for dumping code/comments from source files.

use std::fs;
use std::io;

use process::{find_component, parse_component, Component, FindComponentResult};

mod config;
pub mod presets;
pub mod process;
pub use config::*;
mod format;
pub use format::*;

/// Run the tool
///
/// On success, returns the output of the tool as a vector of lines.
/// On failure, returns an error message.
pub fn execute(file: &str, search_path: &[String], config: &Config) -> Result<Vec<String>, String> {
    let result = match search_file(file, search_path, config) {
        Ok(result) => result,
        Err(e) => return Err(format!("io error while processing file {}: {}", file, e)),
    };

    match result {
        FindComponentResult::NotFound(term) => {
            Err(format!("No component found matching \"{term}\""))
        }
        FindComponentResult::Multiple(matched_children, term) => {
            for matched in matched_children {
                for line in config.format.format(&matched) {
                    eprintln!("{}", line);
                }
            }
            Err(format!("Multiple components found matching \"{term}\". The matched components are shown above."))
        }
        FindComponentResult::Found(component, context) => {
            let output = if config.include_context {
                config.format.format_with_context(&component, &context)
            } else {
                config.format.format(&component)
            };
            Ok(output)
        }
    }
}

/// Search for a component in a file
pub fn search_file(
    file_path: &str,
    search_path: &[String],
    config: &Config,
) -> io::Result<FindComponentResult> {
    let component = parse_file(file_path, config)?;
    println!("component: {:#?}", component);

    Ok(find_component(&component, search_path, config))
}

/// Parse a file into a component
pub fn parse_file(path: &str, config: &Config) -> io::Result<Component> {
    let file_lines = fs::read_to_string(path)?
        .lines()
        .map(|s| s.to_string())
        .collect();

    Ok(parse_component(vec![], file_lines, 0, true, config))
}
