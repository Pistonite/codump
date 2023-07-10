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
