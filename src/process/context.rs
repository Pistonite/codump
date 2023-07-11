//! Logic for converting a component to a context

use crate::process::Component;

/// Context for a component
///
/// Context is used to prevent expensive cloning of the entire component tree
/// when the children are not needed
#[derive(Debug, Clone)]
pub struct Context {
    /// Outer comments
    pub outer_comments: Vec<String>,
    /// Beginning body line
    pub begin_body_line: String,
    /// Inner comments
    pub inner_comments: Vec<String>,
    /// Indentation of the component
    pub indent: usize,
    /// End body lines
    pub end_body_line: String,
}

/// Implementation of Context
impl Context {
    /// Create a context from a component
    pub fn from_component(component: &Component, include_comments: bool) -> Self {
        let begin_body_line = component.body_lines.first().cloned().unwrap_or_default();
        let end_body_line = component.body_lines.last().cloned().unwrap_or_default();
        if include_comments {
            Self {
                outer_comments: component.outer_comments.clone(),
                begin_body_line,
                inner_comments: component.inner_comments.clone(),
                indent: component.indent,
                end_body_line,
            }
        } else {
            Self {
                outer_comments: vec![],
                begin_body_line,
                inner_comments: vec![],
                indent: component.indent,
                end_body_line,
            }
        }
    }
}
