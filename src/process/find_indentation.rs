//! Logic and test for finding the indentation of a block

/// Find the indentation of a block
///
/// The indentation is determined by the first line with leading whitespaces and content after the
/// leading whitespace
///
/// If body lines are empty or none of the lines have leading whitespaces, 0 is returned
pub fn find_indentation(lines: &[String]) -> usize {
    for line in lines {
        match line.find(|x| !super::is_indent_char(x)) {
            None | Some(0) => continue,
            Some(i) => return i,
        }
    }
    0
}

#[cfg(test)]
mod ut {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(find_indentation(&[]), 0);
    }

    #[test]
    fn test_noindent_1() {
        assert_eq!(find_indentation(&["abc".to_string(),]), 0);
    }

    #[test]
    fn test_noindent_many() {
        assert_eq!(
            find_indentation(&["abc".to_string(), "abc2".to_string(),]),
            0
        );
    }

    #[test]
    fn test_indent_first_line() {
        assert_eq!(
            find_indentation(&[" abc".to_string(), "  abc".to_string(), "abc2".to_string(),]),
            1
        );
    }

    #[test]
    fn test_indent_notfirst_line() {
        assert_eq!(
            find_indentation(&["abc".to_string(), "  abc".to_string(), "abc2".to_string(),]),
            2
        );
    }

    #[test]
    fn test_indent_tabs() {
        assert_eq!(
            find_indentation(&["abc".to_string(), "\tabc".to_string(), "abc2".to_string(),]),
            1
        );
    }

    #[test]
    fn test_indent_mix() {
        assert_eq!(
            find_indentation(&["abc".to_string(), "\t abc".to_string(), "abc2".to_string(),]),
            2
        );
    }
}
