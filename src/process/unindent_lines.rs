//! Logic and tests for unindenting lines

/// Unindent the body and remove lines with no leading spaces or tabs
///
/// This returns a new copy with the body lines unindented.
/// The input body lines are not modified.
///
/// This function does not check for indentation. It mechanically removes the first `indent` bytes.
/// If the indent is not at character boundary, the code will panic
pub fn unindent_lines(lines: &[String], indent: usize) -> Vec<String> {
    if indent == 0 {
        return lines.to_vec();
    }

    lines.iter().filter_map(|line| {
        if line.starts_with(|c: char| c == ' ' || c == '\t') {
            Some(line[indent..].to_string())
        } else {
            None
        }
    }).collect()
}

#[cfg(test)]
mod ut {
    use super::*;

    #[test]
    fn test_empty() {
        let expected: Vec<String> = vec![];
        assert_eq!(unindent_lines(&[], 0), expected);
    }

    #[test]
    fn test_indent_0() {
        let expected: Vec<String> = vec![
            "abc".to_string(),
            "abc2".to_string(),
        ];
        assert_eq!(unindent_lines(&expected, 0), expected);
    }

    #[test]
    fn test_indent_1_no_other() {
        let input: Vec<String> = vec![
            " abc".to_string(),
            "\tabc2".to_string(),
        ];
        let expected: Vec<String> = vec![
            "abc".to_string(),
            "abc2".to_string(),
        ];
        assert_eq!(unindent_lines(&input, 1), expected);
    }

    #[test]
    fn test_indent_1_with_more() {
        let input: Vec<String> = vec![
            " abc".to_string(),
            "    abc".to_string(),
            "\tabc2".to_string(),
        ];
        let expected: Vec<String> = vec![
            "abc".to_string(),
            "   abc".to_string(),
            "abc2".to_string(),
        ];
        assert_eq!(unindent_lines(&input, 1), expected);
    }

    #[test]
    fn test_indent_1_with_noindent() {
        let input: Vec<String> = vec![
            " abc".to_string(),
            "abc".to_string(),
            "\tabc2".to_string(),
        ];
        let expected: Vec<String> = vec![
            "abc".to_string(),
            "abc2".to_string(),
        ];
        assert_eq!(unindent_lines(&input, 1), expected);
    }

    #[test]
    fn test_indent_4_no_other() {
        let input: Vec<String> = vec![
            "    abc".to_string(),
            "\t   abc2".to_string(),
        ];
        let expected: Vec<String> = vec![
            "abc".to_string(),
            "abc2".to_string(),
        ];
        assert_eq!(unindent_lines(&input, 4), expected);
    }

    #[test]
    fn test_indent_4_with_more() {
        let input: Vec<String> = vec![
            " \t\t   abcdef".to_string(),
            "    abc".to_string(),
            "\tabc2".to_string(),
        ];
        let expected: Vec<String> = vec![
            "  abcdef".to_string(),
            "abc".to_string(),
            "2".to_string(),
        ];
        assert_eq!(unindent_lines(&input, 4), expected);
    }

    #[test]
    fn test_indent_4_with_noindent() {
        let input: Vec<String> = vec![
            "abc".to_string(),
            " abc".to_string(),
            "abc".to_string(),
            "\tabc2".to_string(),
            "abc".to_string(),
        ];
        let expected: Vec<String> = vec![
            "".to_string(),
            "2".to_string(),
        ];
        assert_eq!(unindent_lines(&input, 4), expected);
    }

}
