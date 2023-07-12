//! Logic and tests for converting body lines to summary view

/// Convert lines to summary view
///
/// Consecutive lines or starts with a space or a tab will be replaced with `...` with the
/// specified indent. Empty lines inside indented region will be ignored while empty lines
/// outside of indented region will be preserved.
///
/// If exclude range is set, the lines in that range will be untouched.
///
/// If indent is 0, this returns the vector as is
///
/// The exclude range has inclusive start and exclusive end.
pub fn summarize_lines(
    lines: &[String],
    indent: usize,
    exclude: Option<(usize, usize)>,
) -> Vec<String> {
    let mut output = vec![];
    let mut is_in_indent = false;
    for (i, line) in lines.iter().enumerate() {
        if line.is_empty() && is_in_indent {
            continue;
        }
        let should_ellipsize = line.starts_with(super::is_indent_char)
            && match exclude {
                Some((start, end)) => i < start || i >= end,
                None => true,
            };
        if should_ellipsize {
            if !is_in_indent {
                output.push(super::indent_string("...", indent));
                is_in_indent = true;
            }
        } else {
            output.push(line.clone());
            is_in_indent = false;
        }
    }

    output
}

/// Tests for summarize_lines
#[cfg(test)]
mod ut {
    use super::*;

    #[test]
    fn test_empty() {
        let expected: Vec<String> = vec![];
        assert_eq!(summarize_lines(&[], 0, None), expected);
    }

    #[test]
    fn test_no_indent() {
        let input = vec!["abc".to_string(), "bcd".to_string(), "cde".to_string()];
        assert_eq!(summarize_lines(&input, 0, None), input);
    }

    #[test]
    fn test_indent_single_begin() {
        let input = vec![" abc".to_string(), "bcd".to_string(), "cde".to_string()];
        let expected = vec!["    ...".to_string(), "bcd".to_string(), "cde".to_string()];
        assert_eq!(summarize_lines(&input, 4, None), expected);
    }

    #[test]
    fn test_indent_single_middle() {
        let input = vec!["bcd".to_string(), " abc".to_string(), "cde".to_string()];
        let expected = vec!["bcd".to_string(), "    ...".to_string(), "cde".to_string()];
        assert_eq!(summarize_lines(&input, 4, None), expected);
    }

    #[test]
    fn test_indent_single_end() {
        let input = vec!["bcd".to_string(), "cde".to_string(), " abc".to_string()];
        let expected = vec!["bcd".to_string(), "cde".to_string(), "    ...".to_string()];
        assert_eq!(summarize_lines(&input, 4, None), expected);
    }

    #[test]
    fn test_indent_multi_begin() {
        let input = vec![
            " abc".to_string(),
            "  abc".to_string(),
            "bcd".to_string(),
            "cde".to_string(),
        ];
        let expected = vec!["    ...".to_string(), "bcd".to_string(), "cde".to_string()];
        assert_eq!(summarize_lines(&input, 4, None), expected);
    }

    #[test]
    fn test_indent_multi_middle() {
        let input = vec![
            "bcd".to_string(),
            " abc".to_string(),
            "  abc".to_string(),
            "cde".to_string(),
        ];
        let expected = vec!["bcd".to_string(), "    ...".to_string(), "cde".to_string()];
        assert_eq!(summarize_lines(&input, 4, None), expected);
    }

    #[test]
    fn test_indent_multi_end() {
        let input = vec![
            "bcd".to_string(),
            "cde".to_string(),
            " abc".to_string(),
            "  abc".to_string(),
        ];
        let expected = vec!["bcd".to_string(), "cde".to_string(), "    ...".to_string()];
        assert_eq!(summarize_lines(&input, 4, None), expected);
    }

    #[test]
    fn test_indent_multi_many1() {
        let input = vec![
            " abc".to_string(),
            "  abc".to_string(),
            "bcd".to_string(),
            "        abc".to_string(),
            "cde".to_string(),
            " abc".to_string(),
            "  abc".to_string(),
        ];
        let expected = vec![
            "    ...".to_string(),
            "bcd".to_string(),
            "    ...".to_string(),
            "cde".to_string(),
            "    ...".to_string(),
        ];
        assert_eq!(summarize_lines(&input, 4, None), expected);
    }

    #[test]
    fn test_indent_multi_many2() {
        let input = vec![
            "bcd".to_string(),
            " abc".to_string(),
            "  abc".to_string(),
            "cde".to_string(),
            "     abc".to_string(),
        ];
        let expected = vec![
            "bcd".to_string(),
            "    ...".to_string(),
            "cde".to_string(),
            "    ...".to_string(),
        ];
        assert_eq!(summarize_lines(&input, 4, None), expected);
    }

    #[test]
    fn test_indent_multi_many3() {
        let input = vec![
            "  abc".to_string(),
            "bcd".to_string(),
            " abc".to_string(),
            " abc".to_string(),
            "cde".to_string(),
        ];
        let expected = vec![
            "    ...".to_string(),
            "bcd".to_string(),
            "    ...".to_string(),
            "cde".to_string(),
        ];
        assert_eq!(summarize_lines(&input, 4, None), expected);
    }

    #[test]
    fn test_exclude_unindent_to_indent() {
        let input = vec![
            "abc".to_string(),
            "bcd".to_string(),
            " abc".to_string(),
            " abc".to_string(),
            " abc".to_string(),
            "cde".to_string(),
        ];
        let expected = vec![
            "abc".to_string(),
            "bcd".to_string(),
            " abc".to_string(),
            " abc".to_string(),
            "  ...".to_string(),
            "cde".to_string(),
        ];
        assert_eq!(summarize_lines(&input, 2, Some((2, 4))), expected);
    }

    #[test]
    fn test_exclude_indent_to_indent() {
        let input = vec![
            "abc".to_string(),
            "bcd".to_string(),
            " abc".to_string(),
            " abc".to_string(),
            " abc".to_string(),
            " abc".to_string(),
            "cde".to_string(),
        ];
        let expected = vec![
            "abc".to_string(),
            "bcd".to_string(),
            "  ...".to_string(),
            " abc".to_string(),
            " abc".to_string(),
            "  ...".to_string(),
            "cde".to_string(),
        ];
        assert_eq!(summarize_lines(&input, 2, Some((3, 5))), expected);
    }

    #[test]
    fn test_exclude_indent_to_unindent() {
        let input = vec![
            "abc".to_string(),
            "bcd".to_string(),
            " abc".to_string(),
            " abc".to_string(),
            " abc".to_string(),
            " abc".to_string(),
            " abc".to_string(),
            "cde".to_string(),
        ];
        let expected = vec![
            "abc".to_string(),
            "bcd".to_string(),
            "  ...".to_string(),
            " abc".to_string(),
            " abc".to_string(),
            " abc".to_string(),
            "cde".to_string(),
        ];
        assert_eq!(summarize_lines(&input, 2, Some((4, 7))), expected);
    }

    #[test]
    fn test_exclude_unindent_to_unindent() {
        let input = vec![
            " abc".to_string(),
            "bcd".to_string(),
            "bcd".to_string(),
            " cde".to_string(),
        ];
        let expected = vec![
            "  ...".to_string(),
            "bcd".to_string(),
            "bcd".to_string(),
            "  ...".to_string(),
        ];
        assert_eq!(summarize_lines(&input, 2, Some((1, 2))), expected);
    }

    #[test]
    fn test_empty_line() {
        let input = vec![
            " abc".to_string(),
            "".to_string(),
            "bcd".to_string(),
            " cde".to_string(),
        ];
        let expected = vec!["  ...".to_string(), "bcd".to_string(), "  ...".to_string()];
        assert_eq!(summarize_lines(&input, 2, None), expected);
    }

    #[test]
    fn test_empty_line_noindent() {
        let input = vec![
            "abc".to_string(),
            "".to_string(),
            "bcd".to_string(),
            " cde".to_string(),
        ];
        let expected = vec![
            "abc".to_string(),
            "".to_string(),
            "bcd".to_string(),
            "  ...".to_string(),
        ];
        assert_eq!(summarize_lines(&input, 2, None), expected);
    }
}
