//! Logic and data structures for parsing/finding a component from lines

use crate::process::{find_comments, find_indentation, unindent_lines, CommentPattern};
use crate::Config;

/// Data of a component
#[derive(Debug, Clone)]
pub struct Component {
    /// If the component is the root file
    pub is_root: bool,
    /// Outer comments
    pub outer_comments: Vec<String>,
    /// Body lines (unparsed)
    ///
    /// Includes inner comment lines
    pub body_lines: Vec<String>,
    /// Inner comments
    ///
    /// These are unindented
    pub inner_comments: Vec<String>,
    /// Range of lines for inner comments in the body lines
    pub inner_comments_range: Option<(usize, usize)>,
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
    is_root: bool,
    config: &Config,
) -> Component {
    // need to find indent range first since the range will be
    // different when unindented
    let inner_comments_range = find_comments(&body_lines, &config.inner_comments, indent);
    let unindented_body_lines = unindent_lines(&body_lines, indent)
        .into_iter()
        .filter(|line| {
            !config
                .ignore_lines
                .iter()
                .any(|pattern| pattern.is_match(line))
        })
        .collect::<Vec<_>>();

    let mut comment_end = 0;

    // find inner comments
    let inner_comments = if let Some((start, end)) =
        find_comments(&unindented_body_lines, &config.inner_comments, 0)
    {
        comment_end = end;
        unindented_body_lines[start..end].to_vec()
    } else {
        vec![]
    };

    // skip to the first child
    let (mut comment_start, mut comment_end) = if let Some((start, end)) =
        find_next_child_outer_comment(
            &unindented_body_lines[comment_end..],
            &config.outer_comments,
        ) {
        (comment_end + start, comment_end + end)
    } else {
        // no children
        return Component {
            is_root,
            outer_comments,
            body_lines,
            inner_comments,
            inner_comments_range,
            children: vec![],
            indent,
        };
    };

    let mut children = vec![];

    while comment_end < unindented_body_lines.len() {
        // extract child lines
        let child_outer_comments = unindented_body_lines[comment_start..comment_end].to_vec();
        // try finding next comment
        let child_body_lines = if let Some((start, end)) = find_next_child_outer_comment(
            &unindented_body_lines[comment_end..],
            &config.outer_comments,
        ) {
            let child_body_lines = unindented_body_lines[comment_end..comment_end + start].to_vec();
            // update indices
            comment_start = comment_end + start;
            comment_end += end;
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
            false,
            config,
        ));
    }

    Component {
        is_root,
        outer_comments,
        body_lines,
        inner_comments,
        inner_comments_range,
        children,
        indent,
    }
}

/// Helper for locating the next child's outer comments.
///
/// Returns the start and end indices of the next child's outer comments,
/// or None if there are no more children.
///
/// The criteria for a child is that it has outer comments, and the next line
/// after the outer comment exists, is not empty, and has no indent.
fn find_next_child_outer_comment(
    lines: &[String],
    pattern: &CommentPattern,
) -> Option<(usize, usize)> {
    let mut comment_end = 0;
    while comment_end < lines.len() {
        // find next outer comment from comment_end
        if let Some((start, end)) = find_comments(&lines[comment_end..], pattern, 0) {
            // check if next line is not empty and not indented
            if let Some(next_line) = lines.get(comment_end + end) {
                if !next_line.is_empty() && !next_line.starts_with(super::is_indent_char) {
                    // found next child
                    return Some((comment_end + start, comment_end + end));
                }
            }

            // keep finding next comment
            comment_end += end;
        } else {
            break;
        }
    }
    // outer comment not found
    None
}
