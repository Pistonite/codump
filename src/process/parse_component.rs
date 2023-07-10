//! Logic and data structures for parsing/finding a component from lines

use crate::Config;
use crate::process::{
    find_comments, 
    find_indentation, 
    unindent_lines, 
};

/// Data of a component
#[derive(Debug, Clone)]
pub struct Component {
    /// Outer comments
    pub outer_comments: Vec<String>,
    /// Body lines (unparsed)
    ///
    /// May include inner comment lines
    pub body_lines: Vec<String>,
    /// Inner comments
    ///
    /// These will keep their indentation
    pub inner_comments: Vec<String>,
    /// Child components
    pub children: Vec<Component>,
    /// Indentation from parent
    pub indent: usize,
}

/// Parse a component body.
///
/// The body lines passed in should be the indented lines that will be
/// stored directly in the Component.
/// The indent is used to process the lines, and the parsing is done on the
/// unindented lines.
pub fn parse_component(
    outer_comments: Vec<String>,
    body_lines: Vec<String>,
    indent: usize,
    config: &Config,
) -> Component {
    let unindented_body_lines = unindent_lines(&body_lines, indent)
        .into_iter()
        .filter(|line| {
            !config
                .ignore_lines
                .iter()
                .any(|pattern| pattern.is_match(line))
        })
        .collect::<Vec<_>>();
    // current first line
    let mut comment_end = 0;
    // find inner comments
    let inner_comments =
        if let Some((start, end)) = find_comments(&unindented_body_lines, &config.inner_comments) {
            comment_end = end;
            unindented_body_lines[start..end].to_vec()
        } else {
            vec![]
        };

    // skip to the first child
    let (mut comment_start, mut comment_end) = if let Some((start, end)) = find_comments(
        &unindented_body_lines[comment_end..],
        &config.outer_comments,
    ) {
        (comment_end + start, comment_end + end)
    } else {
        // no children
        return Component {
            outer_comments,
            body_lines,
            inner_comments,
            children: vec![],
            indent,
        };
    };

    let mut children = vec![];

    while comment_end < unindented_body_lines.len() {
        // extract child lines
        let child_outer_comments = unindented_body_lines[comment_start..comment_end].to_vec();
        // try finding next comment
        let child_body_lines = if let Some((start, end)) = find_comments(
            &unindented_body_lines[comment_end..],
            &config.outer_comments,
        ) {
            let child_body_lines = unindented_body_lines[comment_end..comment_end + start].to_vec();
            // update indices
            comment_start = comment_end + start;
            comment_end = comment_end + end;
            child_body_lines
        } else {
            let child_body_lines = unindented_body_lines[comment_end..].to_vec();
            comment_end = unindented_body_lines.len();
            child_body_lines
        };
        let child_indent = find_indentation(&child_body_lines);
        children.push(parse_component(
            child_outer_comments,
            child_body_lines,
            child_indent,
            config,
        ));
    }

    Component {
        outer_comments,
        body_lines,
        inner_comments,
        children,
        indent,
    }
}

