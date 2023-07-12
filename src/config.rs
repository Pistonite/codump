//! Configuration for the tool
//!
//! Including both internal config data structure and CLI args

#[cfg(feature = "cli")]
use clap::Parser;

use crate::presets::Preset;
use crate::process::CommentPattern;
use crate::Format;
use regex::Regex;

/// Command line interface
#[derive(Debug)]
#[cfg_attr(feature = "cli", derive(Parser))]
#[cfg_attr(
    feature = "cli",
    command(
        bin_name = "codump",
        about,
        version,
        author,
        arg_required_else_help = true
    )
)]
pub struct CliArgs {
    /// The input file to parse
    #[cfg_attr(feature = "cli", arg(required = true))]
    pub file: String,

    /// The component search path
    ///
    /// Each path is a case-sensitive substring used to search for components at that level.
    /// The first line of the code after the doc comments is searched for the substring.
    #[cfg_attr(feature = "cli", arg(required = true))]
    pub search_path: Vec<String>,

    /// Outer single line comment regex
    #[cfg_attr(feature = "cli", arg(long))]
    outer: Option<String>,

    /// Outer multi line comment start regex
    #[cfg_attr(feature = "cli", arg(long))]
    outer_start: Option<String>,

    /// Outer multi line comment end regex
    #[cfg_attr(feature = "cli", arg(long))]
    outer_end: Option<String>,

    /// Inner single line comment regex
    #[cfg_attr(feature = "cli", arg(long))]
    inner: Option<String>,

    /// Inner multi line comment start regex
    #[cfg_attr(feature = "cli", arg(long))]
    inner_start: Option<String>,

    /// Inner multi line comment end regex
    #[cfg_attr(feature = "cli", arg(long))]
    inner_end: Option<String>,

    /// Pattern for lines that should be ignored
    #[cfg_attr(feature = "cli", arg(long, short))]
    ignore: Vec<String>,

    /// Format for the output
    #[cfg_attr(feature = "cli", arg(long, short, default_value = "summary"))]
    format: Format,

    /// Use a preset configuration
    ///
    /// If both presets and individual options are set,
    /// the individual options will override the corresponding part of the preset.
    /// The rest of the preset will still be used.
    #[cfg_attr(feature = "cli", arg(long, short))]
    preset: Option<Preset>,

    /// Print context
    ///
    /// Context is the parent components of the found component
    #[cfg_attr(feature = "cli", arg(long, short))]
    context: bool,

    /// Print context comments
    ///
    /// Print the comments of the parents along with the context (implies --context)
    #[cfg_attr(feature = "cli", arg(long, short = 'C'))]
    context_comments: bool,
}

/// Internal config data structure
#[derive(Debug, Clone)]
pub struct Config {
    /// Pattern for matching outer comments
    pub outer_comments: CommentPattern,
    /// Pattern for matching inner comments
    pub inner_comments: CommentPattern,
    /// Pattern of lines to ignore
    pub ignore_lines: Vec<Regex>,
    /// If context should be included
    pub include_context: bool,
    /// If context should include comments
    pub context_include_comments: bool,
    /// Format of the output
    pub format: Format,
}

impl TryFrom<CliArgs> for Config {
    type Error = String;

    fn try_from(args: CliArgs) -> Result<Self, Self::Error> {
        let (outer_comments, inner_comments) = match args.preset {
            Some(preset) => {
                let (mut outer, mut inner) = preset.get_patterns();
                if let Some(v) = args.outer {
                    outer.single_line = parse_regex(&v)?;
                }
                if let Some(v) = args.outer_start {
                    outer.multi_start = Some(parse_regex(&v)?);
                }
                if let Some(v) = args.outer_end {
                    outer.multi_end = parse_regex(&v)?;
                }
                if let Some(v) = args.inner {
                    inner.single_line = parse_regex(&v)?;
                }
                if let Some(v) = args.inner_start {
                    inner.multi_start = Some(parse_regex(&v)?);
                }
                if let Some(v) = args.inner_end {
                    inner.multi_end = parse_regex(&v)?;
                }
                (outer, inner)
            }
            None => parse_comment_pattern_from_args(&args)?,
        };
        let mut ignore_lines = vec![];
        for line in args.ignore {
            ignore_lines.push(parse_regex(&line)?);
        }

        Ok(Config {
            outer_comments,
            inner_comments,
            ignore_lines,
            include_context: args.context || args.context_comments,
            context_include_comments: args.context_comments,
            format: args.format,
        })
    }
}

fn parse_comment_pattern_from_args(
    args: &CliArgs,
) -> Result<(CommentPattern, CommentPattern), String> {
    Ok((
        CommentPattern {
            single_line: parse_comment_pattern(args.outer.as_ref())?,
            multi_start: Some(parse_comment_pattern(args.outer_start.as_ref())?),
            multi_end: parse_comment_pattern(args.outer_end.as_ref())?,
        },
        CommentPattern {
            single_line: parse_comment_pattern(args.inner.as_ref())?,
            multi_start: Some(parse_comment_pattern(args.inner_start.as_ref())?),
            multi_end: parse_comment_pattern(args.inner_end.as_ref())?,
        },
    ))
}

fn parse_comment_pattern(pattern: Option<&String>) -> Result<Regex, String> {
    match pattern {
        Some(s) => parse_regex(s),
        None => Err("Comment pattern missing. Either use a --preset or specify all the --outer* and --inner* arguments. See --help for more.".to_string()),
    }
}

fn parse_regex(s: &str) -> Result<Regex, String> {
    Regex::new(s).map_err(|_| format!("Invalid regex \"{}\". See --help for more.", s))
}
