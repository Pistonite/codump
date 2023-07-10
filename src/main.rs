//! Entry point for the executable

use clap::Parser;
use regex::Regex;
use codump::{Format, Config};
use codump::presets::Preset;
use codump::process::{CommentPattern, FindComponentResult};

/// Command line interface
#[derive(Debug, Parser)]
#[command(
    bin_name = "codump",
    about,
    version,
    author,
    arg_required_else_help = true
)]
struct CliArgs {
    /// The input file to parse
    #[arg(required = true)]
    file: String,

    /// The component search path
    ///
    /// Each path is a case-sensitive substring used to search for components at that level.
    /// The first line of the code after the doc comments is searched for the substring.
    #[arg(required = true)]
    search_path: Vec<String>,

    /// Outer single line comment regex
    #[arg(long)]
    outer: Option<String>,

    /// Outer multi line comment start regex
    #[arg(long)]
    outer_start: Option<String>,

    /// Outer multi line comment end regex
    #[arg(long)]
    outer_end: Option<String>,

    /// Inner single line comment regex
    #[arg(long)]
    inner: Option<String>,

    /// Inner multi line comment start regex
    #[arg(long)]
    inner_start: Option<String>,

    /// Inner multi line comment end regex
    #[arg(long)]
    inner_end: Option<String>,

    /// Pattern for lines that should be ignored
    #[arg(long, short)]
    ignore: Vec<String>,

    /// Format for the output
    #[arg(long, short, default_value = "summary")]
    format: Format,

    /// Use a preset configuration
    ///
    /// If both presets and individual options are set,
    /// the individual options will override the corresponding part of the preset.
    /// The rest of the preset will still be used.
    #[arg(long, short)]
    preset: Option<Preset>,

    /// Print context
    ///
    /// Context is the parent components of the found component
    #[arg(long, short)]
    context: bool,

    /// Print context comments
    ///
    /// Print the comments of the parent components. Only applies if `--context` is also set.
    #[arg(long, short = 'C')]
    context_comments: bool,
}

fn main() {
    if let Err(e) = main_internal() {
        eprintln!("error: {}", e);
    }
}

/// Main function
fn main_internal() -> Result<(), String> {
    let args = CliArgs::parse();
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
        None => {
            parse_comment_pattern_from_args(&args)?
        }
    };

    let mut ignore_lines = vec![];
    for line in args.ignore {
        ignore_lines.push(parse_regex(&line)?);
    }

    let config = Config {
        outer_comments,
        inner_comments,
        ignore_lines,
        context_include_comments: args.context_comments,
    };

    let result = match codump::search_file(&args.file, &args.search_path, &config) {
        Ok(result) => result,
        Err(e) => return Err(format!("io error while processing file {}: {}", &args.file, e)),
    };

    match result {
        NFindComponentResul::NotFound(term) => {
            return Err(format!("No component found matching \"{}\"", term));
        },
        FindComponentResult::Multiple(matched, term) => {
            return Err(format!("Multiple components found matching \"{}\":\n{}", term, args.format.format(&matched)));
        },

        Some(result) => {
            println!("{}", args.format.format(&result));
        }
        None => {
            println!("No component found");
        }
    }

    Ok(())
}

fn parse_comment_pattern_from_args(args: &CliArgs) -> Result<(CommentPattern, CommentPattern), String> {
    Ok((CommentPattern {
        single_line: parse_comment_pattern(args.outer.as_ref())?,
        multi_start: Some(parse_comment_pattern(args.outer_start.as_ref())?),
        multi_end: parse_comment_pattern(args.outer_end.as_ref())?,
    },
    CommentPattern {
        single_line: parse_comment_pattern(args.inner.as_ref())?,
        multi_start: Some(parse_comment_pattern(args.inner_start.as_ref())?),
        multi_end: parse_comment_pattern(args.inner_end.as_ref())?,
    }))
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
