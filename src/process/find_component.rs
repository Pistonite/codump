//! Logic for finding a component in the parsed component tree

use crate::Config;
use crate::process::{
    Component, Context,
};

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

    let matched_children = component
        .children
        .iter()
        .filter(|child| match child.body_lines.first() {
            Some(line) => line.contains(&search_path[0]),
            None => false,
        })
        .collect::<Vec<_>>();

    match matched_children.len() {
        0 => FindComponentResult::NotFound(search_path[0].clone()),
        1 => {
            let result = find_component(matched_children[0], &search_path[1..], config);
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
        _ => {
            let matched_children = matched_children
                .into_iter()
                .map(|c| c.clone())
                .collect::<Vec<_>>();

            FindComponentResult::Multiple(matched_children, search_path[0].clone())
        }
    }
}
