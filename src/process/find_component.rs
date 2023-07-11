//! Logic for finding a component in the parsed component tree

use crate::process::{Component, Context};
use crate::Config;

/// Result for calling find_component
#[derive(Debug, Clone)]
pub enum FindComponentResult {
    /// Found a component uniquely.
    ///
    /// The context returned is the reversed path to the file root.
    /// The last element in the context is the file itself.
    Found(Component, Vec<Context>),
    /// No component found
    ///
    /// Returns the search term that causes the no match.
    NotFound(String),
    /// Found multiple matches at some level.
    ///
    /// Returns all matches at that level, and the search term that causes the multiple match.
    Multiple(Vec<Component>, String),
}

/// Find a component from a root component
///
/// Returns the component itself if search_path is empty.
pub fn find_component(
    component: &Component,
    search_path: &[String],
    config: &Config,
) -> FindComponentResult {
    if search_path.is_empty() {
        return FindComponentResult::Found(component.clone(), vec![]);
    }

    let matched_children = find_children(component, &search_path[0]);

    match matched_children.len() {
        0 => FindComponentResult::NotFound(search_path[0].clone()),
        1 => {
            let result = find_component(&matched_children[0], &search_path[1..], config);
            match result {
                FindComponentResult::Found(comp, mut ctx) => {
                    ctx.push(Context::from_component(
                        component,
                        config.context_include_comments,
                    ));
                    FindComponentResult::Found(comp, ctx)
                }
                _ => result,
            }
        }
        _ => FindComponentResult::Multiple(matched_children, search_path[0].clone()),
    }
}

/// Find children components from a component based on a search string
///
/// The substring is first matched against the first line of each child.
/// If no child is matched, it moves on to the second line, and so on.
///
/// Returns a vector of all matched children.
pub fn find_children(component: &Component, search: &str) -> Vec<Component> {
    let max_lines = component
        .children
        .iter()
        .map(|child| child.body_lines.len())
        .max()
        .unwrap_or(0);

    let mut matched_children = vec![];
    for i in 0..max_lines {
        for child in &component.children {
            if let Some(line) = child.body_lines.get(i) {
                if line.contains(search) {
                    matched_children.push(child.clone());
                }
            }
        }

        if !matched_children.is_empty() {
            return matched_children;
        }
    }

    matched_children
}
