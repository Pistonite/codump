//! Processing logic

mod find_comments;
pub use find_comments::*;
mod find_indentation;
pub use find_indentation::*;
mod unindent_lines;
pub use unindent_lines::*;
mod parse_component;
pub use parse_component::*;
mod context;
pub use context::*;
mod find_component;
pub use find_component::*;
mod summarize_lines;
pub use summarize_lines::*;

/// Helper function to check if a char is a valid indent character
pub fn is_indent_char(c: char) -> bool {
    c == ' ' || c == '\t'
}

/// Helper function to indent a string
pub fn indent_string(s: &str, indent: usize) -> String {
    format!("{:indent$}{s}", "")
}
