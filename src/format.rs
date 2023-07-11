//! Logic for printing output in different formats

#[cfg(feature = "cli")]
use clap::ValueEnum;

use crate::process::{indent_string, summarize_lines, Component, Context};

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

impl Format {
    /// Format a component without context
    pub fn format(&self, component: &Component) -> Vec<String> {
        self.format_with_context(component, &[])
    }

    /// Format a component with context
    pub fn format_with_context(&self, component: &Component, contexts: &[Context]) -> Vec<String> {
        let mut indent: usize = 0;
        let mut output = vec![];
        // add context beginning
        contexts.iter().for_each(|context| {
            context.outer_comments.iter().for_each(|line| {
                output.push(indent_string(line, indent));
            });
            output.push(indent_string(&context.begin_body_line, indent));
            indent += context.indent;
            context.inner_comments.iter().for_each(|line| {
                output.push(indent_string(line, indent));
            });
            output.push(indent_string("...", indent));
        });
        // add component
        let component_lines = match self {
            Format::Summary => format_summary(component),
            Format::Comment => format_comment(component),
            Format::Detail => format_detail(component),
        };

        component_lines.iter().for_each(|line| {
            output.push(indent_string(line, indent));
        });

        // add context ending
        contexts.iter().for_each(|context| {
            output.push(indent_string("...", indent));
            output.push(indent_string(&context.end_body_line, indent));
            indent -= context.indent;
        });

        output
    }
}

/// Format a component in summary format
fn format_summary(component: &Component) -> Vec<String> {
    let mut output = vec![];
    // add outer comments
    component.outer_comments.iter().for_each(|line| {
        output.push(line.clone());
    });
    // add summary
    output.append(&mut summarize_lines(
        &component.body_lines,
        component.indent,
        component.inner_comments_range,
    ));

    output
}

/// Format a component in comment only format
fn format_comment(component: &Component) -> Vec<String> {
    let mut output = vec![];
    // add outer comments
    component.outer_comments.iter().for_each(|line| {
        output.push(line.clone());
    });
    // add inner comments
    component.inner_comments.iter().for_each(|line| {
        output.push(line.clone());
    });

    output
}

/// Format a component in detail format
fn format_detail(component: &Component) -> Vec<String> {
    let mut output = vec![];
    // add outer comments
    component.outer_comments.iter().for_each(|line| {
        output.push(line.clone());
    });
    // add full body, which includes inner comments
    component.body_lines.iter().for_each(|line| {
        output.push(line.clone());
    });

    output
}
