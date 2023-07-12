//! Logic for converting a component to a context

use crate::process::{summarize_lines, Component};

/// Context for a component
///
/// Context is used to prevent expensive cloning of the entire component tree
/// when the children are not needed
#[derive(Debug, Clone)]
pub struct Context {
    /// Outer comments
    pub outer_comments: Vec<String>,
    /// Beginning body lines
    pub begin_body_lines: Vec<String>,
    /// Indentation of the component
    pub indent: usize,
    /// End body lines
    pub end_body_lines: Vec<String>,
}

/// Implementation of Context
impl Context {
    /// Create a context from a component
    pub fn from_component(component: &Component, include_comments: bool) -> Self {
        let begin_body_lines = get_begin_body_lines(component, include_comments);
        let end_body_lines = get_end_body_lines(component);

        if include_comments {
            Self {
                outer_comments: component.outer_comments.clone(),
                begin_body_lines,
                indent: component.indent,
                end_body_lines,
            }
        } else {
            Self {
                outer_comments: vec![],
                begin_body_lines,
                indent: component.indent,
                end_body_lines,
            }
        }
    }
}

/// Check if the last line in the lines exist and is an indented block
fn is_last_indented_block(lines: &[String]) -> bool {
    if let Some(last) = lines.last() {
        if last.starts_with(super::is_indent_char) {
            return true;
        }
    }
    false
}
fn get_begin_body_lines(component: &Component, include_comments: bool) -> Vec<String> {
    if component.is_root {
        let mut l: Vec<String> = if include_comments {
            component
                .inner_comments
                .iter()
                .map(|s| super::indent_string(s, component.indent))
                .collect()
        } else {
            vec![]
        };
        l.push(super::indent_string("...", component.indent));
        return l;
    }
    match component.inner_comments_range {
        Some((start, _)) => {
            let mut l = summarize_lines(&component.body_lines[..start], component.indent, None);
            // add the inner comments if need
            if include_comments {
                component
                    .inner_comments
                    .iter()
                    .for_each(|s| l.push(super::indent_string(s, component.indent)));
                l.push(super::indent_string("...", component.indent));
            } else if !is_last_indented_block(&l) {
                // add the ... block if needed
                l.push(super::indent_string("...", component.indent));
            }
            l
        }
        None => {
            let mut l = summarize_lines(&component.body_lines, component.indent, None);
            // Remove until last ... block
            while !is_last_indented_block(&l) {
                if l.pop().is_none() {
                    break;
                }
            }
            l
        }
    }
}

fn get_end_body_lines(component: &Component) -> Vec<String> {
    if component.is_root {
        return vec![super::indent_string("...", component.indent)];
    }
    let mut l = match component.inner_comments_range {
        Some((_, end)) => summarize_lines(&component.body_lines[end..], component.indent, None),
        None => summarize_lines(&component.body_lines, component.indent, None),
    };
    // only keep the last ... block
    let mut last = vec![];
    while !is_last_indented_block(&l) {
        match l.pop() {
            Some(s) => last.push(s),
            None => break,
        }
    }
    last.push(super::indent_string("...", component.indent));
    last.reverse();
    last
}
