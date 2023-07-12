//! Logic and tests for finding comments from list of lines

use regex::Regex;

/// Regex patterns for comments
///
/// The patterns should match the entire line.
#[derive(Debug, Clone)]
pub struct CommentPattern {
    /// Pattern for single-line comments
    pub single_line: Regex,
    /// Pattern for the first line of multi line comments
    ///
    /// If none, then multi line comments are not supported
    pub multi_start: Option<Regex>,
    /// Pattern for the last line of the multi line comments
    ///
    /// The last line must not be the start line.
    /// If the comment syntax supports single-line block comment, then
    /// the single line regex should support that.
    ///
    /// Ignored if multi_start is None.
    pub multi_end: Regex,
}

/// Locate the first block of comments from list of lines
///
/// Returns the start (inclusive) and end (exclusive) indices.
///
/// The first `indent` bytes are ignored if the line starts with a space or tab
/// . This assumes the first `indent` bytes only contains spaces/tabs
/// (i.e. will panic if cutting off in the middle of unicode character)
pub fn find_comments(
    body_lines: &[String],
    patterns: &CommentPattern,
    indent: usize,
) -> Option<(usize, usize)> {
    let mut is_multi = false;
    let mut start: Option<usize> = None;
    for (i, line) in body_lines.iter().enumerate() {
        let line = if indent >= line.len() {
            ""
        } else {
            &line[indent..]
        };
        match start {
            Some(start) => {
                if is_multi {
                    if patterns.multi_end.is_match(line) {
                        return Some((start, i + 1));
                    }
                } else if !patterns.single_line.is_match(line) {
                    return Some((start, i));
                }
            }
            None => {
                // not found comment yet
                if patterns.single_line.is_match(line) {
                    start = Some(i);
                } else if let Some(pattern) = &patterns.multi_start {
                    if pattern.is_match(line) {
                        // if patterns.multi_end.is_match(line) {
                        //     return Some((i, i + 1));
                        // }
                        start = Some(i);
                        is_multi = true;
                    }
                }
            }
        }
    }

    start.map(|start| (start, body_lines.len()))
}

#[cfg(test)]
mod ut {
    use super::*;

    fn create_test_pattern() -> CommentPattern {
        CommentPattern {
            single_line: Regex::new(r"^///").unwrap(),
            multi_start: Some(Regex::new(r"^/\*\*").unwrap()),
            multi_end: Regex::new(r"\*/\s*$").unwrap(),
        }
    }

    fn create_test_pattern_single_only() -> CommentPattern {
        CommentPattern {
            single_line: Regex::new(r"^///").unwrap(),
            multi_start: None,
            multi_end: Regex::new(r"\*/\s*$").unwrap(),
        }
    }

    #[test]
    fn test_1_empty() {
        let lines = vec![];
        let pattern = create_test_pattern();
        let result = find_comments(&lines, &pattern, 0);
        assert!(result.is_none());
    }

    #[test]
    fn test_1_no_comment() {
        let lines = vec![
            "fn main() {".to_string(),
            "    println!(\"Hello, world!\");".to_string(),
            "}".to_string(),
        ];
        let pattern = create_test_pattern();
        let result = find_comments(&lines, &pattern, 0);
        assert!(result.is_none());
    }

    #[test]
    fn test_1_comment_start_single() {
        let lines = vec![
            "///abcde ".to_string(),
            "fn main() {".to_string(),
            "    println!(\"Hello, world!\");".to_string(),
            "}".to_string(),
        ];
        let pattern = create_test_pattern();
        let (start, end) = find_comments(&lines, &pattern, 0).unwrap();
        assert_eq!(start, 0);
        assert_eq!(end, 1);
    }

    #[test]
    fn test_1_comment_start_single_many() {
        let lines = vec![
            "///abcde ".to_string(),
            "///abcde2 ".to_string(),
            "///abcde23".to_string(),
            "fn main() {".to_string(),
            "    println!(\"Hello, world!\");".to_string(),
            "}".to_string(),
            "///abcde24".to_string(),
        ];
        let pattern = create_test_pattern();
        let (start, end) = find_comments(&lines, &pattern, 0).unwrap();
        assert_eq!(start, 0);
        assert_eq!(end, 3);
    }

    #[test]
    fn test_1_comment_start_single_none_in_between() {
        let lines = vec![
            "///abcde ".to_string(),
            "fn main() {".to_string(),
            "///abcde2 ".to_string(),
            "fn main() {".to_string(),
            "    println!(\"Hello, world!\");".to_string(),
            "}".to_string(),
        ];
        let pattern = create_test_pattern();
        let (start, end) = find_comments(&lines, &pattern, 0).unwrap();
        assert_eq!(start, 0);
        assert_eq!(end, 1);
    }

    #[test]
    fn test_1_comment_start_multi() {
        let lines = vec![
            "/**abcde*/ ".to_string(),
            "fn main() {".to_string(),
            "    println!(\"Hello, world!\");".to_string(),
            "}".to_string(),
        ];
        let pattern = create_test_pattern();
        let (start, end) = find_comments(&lines, &pattern, 0).unwrap();
        assert_eq!(start, 0);
        assert_eq!(end, 4);
    }

    #[test]
    fn test_1_comment_start_multi_many() {
        let lines = vec![
            "/**abcde ".to_string(),
            "///abcde2 ".to_string(),
            "///abcde23".to_string(),
            "///abcde23*/".to_string(),
            "fn main() {".to_string(),
            "    println!(\"Hello, world!\");".to_string(),
            "}".to_string(),
        ];
        let pattern = create_test_pattern();
        let (start, end) = find_comments(&lines, &pattern, 0).unwrap();
        assert_eq!(start, 0);
        assert_eq!(end, 4);
    }

    #[test]
    fn test_1_comment_start_multi_none_in_between() {
        let lines = vec![
            "/**abcde ".to_string(),
            "///abcde23*/".to_string(),
            "/**abcde ".to_string(),
            "///abcde2 ".to_string(),
            "///abcde23".to_string(),
            "///abcde23*/".to_string(),
            "///abcde2 ".to_string(),
            "fn main() {".to_string(),
            "    println!(\"Hello, world!\");".to_string(),
            "}".to_string(),
        ];
        let pattern = create_test_pattern();
        let (start, end) = find_comments(&lines, &pattern, 0).unwrap();
        assert_eq!(start, 0);
        assert_eq!(end, 2);
    }

    #[test]
    fn test_1_comment_end_single() {
        let lines = vec![
            "fn main() {".to_string(),
            "    println!(\"Hello, world!\");".to_string(),
            "}".to_string(),
            "///abcde ".to_string(),
        ];
        let pattern = create_test_pattern();
        let (start, end) = find_comments(&lines, &pattern, 0).unwrap();
        assert_eq!(start, 3);
        assert_eq!(end, 4);
    }

    #[test]
    fn test_1_comment_end_single_many() {
        let lines = vec![
            "fn main() {".to_string(),
            "    println!(\"Hello, world!\");".to_string(),
            "}".to_string(),
            "///abcde23".to_string(),
            "///abcde24".to_string(),
        ];
        let pattern = create_test_pattern();
        let (start, end) = find_comments(&lines, &pattern, 0).unwrap();
        assert_eq!(start, 3);
        assert_eq!(end, 5);
    }

    #[test]
    fn test_1_comment_end_single_none_in_between() {
        let lines = vec![
            "fn main() {".to_string(),
            "    println!(\"Hello, world!\");".to_string(),
            "}".to_string(),
            "///abcde ".to_string(),
            "fn main() {".to_string(),
            "///abcde2 ".to_string(),
        ];
        let pattern = create_test_pattern();
        let (start, end) = find_comments(&lines, &pattern, 0).unwrap();
        assert_eq!(start, 3);
        assert_eq!(end, 4);
    }

    #[test]
    fn test_1_comment_end_multi() {
        let lines = vec![
            "fn main() {".to_string(),
            "    println!(\"Hello, world!\");".to_string(),
            "}".to_string(),
            "/**abcde ".to_string(),
            "abcde*/ ".to_string(),
        ];
        let pattern = create_test_pattern();
        let (start, end) = find_comments(&lines, &pattern, 0).unwrap();
        assert_eq!(start, 3);
        assert_eq!(end, 5);
    }

    #[test]
    fn test_1_comment_end_multi_many() {
        let lines = vec![
            "fn main() {".to_string(),
            "    println!(\"Hello, world!\");".to_string(),
            "}".to_string(),
            "/**abcde ".to_string(),
            "abcde*/ ".to_string(),
        ];
        let pattern = create_test_pattern();
        let (start, end) = find_comments(&lines, &pattern, 0).unwrap();
        assert_eq!(start, 3);
        assert_eq!(end, 5);
    }

    #[test]
    fn test_1_comment_end_multi_none_in_between() {
        let lines = vec![
            "fn main() {".to_string(),
            "    println!(\"Hello, world!\");".to_string(),
            "}".to_string(),
            "/**abcde ".to_string(),
            "abcde*/ ".to_string(),
            "fn main() {".to_string(),
            "/**abcde ".to_string(),
            "abcde*/ ".to_string(),
        ];
        let pattern = create_test_pattern();
        let (start, end) = find_comments(&lines, &pattern, 0).unwrap();
        assert_eq!(start, 3);
        assert_eq!(end, 5);
    }

    #[test]
    fn test_single_only() {
        let lines = vec![
            "/**abcde ".to_string(),
            "abcde*/ ".to_string(),
            "fn main() {".to_string(),
            "    println!(\"Hello, world!\");".to_string(),
            "}".to_string(),
            "///abcde ".to_string(),
            "fn main() {".to_string(),
            "///abcde2 ".to_string(),
        ];
        let pattern = create_test_pattern_single_only();
        let (start, end) = find_comments(&lines, &pattern, 0).unwrap();
        assert_eq!(start, 5);
        assert_eq!(end, 6);
    }

    #[test]
    fn test_with_indent() {
        let lines = vec![
            "/**abcde ".to_string(),
            "abcde*/ ".to_string(),
            "  /**abcde ".to_string(),
            "  abcde*/ ".to_string(),
        ];
        let pattern = create_test_pattern();
        let (start, end) = find_comments(&lines, &pattern, 2).unwrap();
        assert_eq!(start, 2);
        assert_eq!(end, 4);
    }

    #[test]
    fn test_with_indent_short_line() {
        let lines = vec![
            "a".to_string(),
            "  /**abcde ".to_string(),
            "  abcde*/ ".to_string(),
        ];
        let pattern = create_test_pattern();
        let (start, end) = find_comments(&lines, &pattern, 2).unwrap();
        assert_eq!(start, 1);
        assert_eq!(end, 3);
    }

    #[test]
    fn test_with_indent_short_line_end() {
        let lines = vec![
            "  /// something".to_string(),
            "h".to_string(),
            "  /// something".to_string(),
        ];
        let pattern = create_test_pattern();
        let (start, end) = find_comments(&lines, &pattern, 2).unwrap();
        assert_eq!(start, 0);
        assert_eq!(end, 1);
    }
}
