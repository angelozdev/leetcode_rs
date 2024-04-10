fn compare(mut start: usize, mut end: usize, slice: &[u8]) -> bool {
    while start < end {
        if slice[start] != slice[end] {
            return false;
        }
        start += 1;
        end -= 1;
    }
    true
}

/// Determines if a string can be a palindrome after removing at most one character.
///
/// This function iterates from both ends of the string towards the center,
/// checking for character equality. If a mismatch is found, it explores two
/// scenarios: removing one character from the start or the end, then checking
/// if the resulting substring can form a palindrome.
///
/// # Parameters
///
/// * `s` - The input string to check.
///
/// # Returns
///
/// * `true` if the input can form a palindrome by removing at most one character;
///   otherwise, `false`.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use leetcode_rs::easy::valid_palindrome_ii::valid_palindrome;
/// assert_eq!(valid_palindrome("aba".to_string()), true);
/// assert_eq!(valid_palindrome("abca".to_string()), true);
/// assert_eq!(valid_palindrome("abc".to_string()), false);
/// ```
///
/// # Constraints
///
/// * The length of `s` is in the range [1, 10^5].
/// * `s` consists only of lowercase English letters.
pub fn valid_palindrome(s: String) -> bool {
    if s.is_empty() {
        return true;
    }

    let s_as_vec = s.as_bytes();
    let (mut start, mut end) = (0, s.len() - 1);

    while start < end {
        if s_as_vec[start] != s_as_vec[end] {
            return compare(start + 1, end, &s_as_vec) || compare(start, end - 1, &s_as_vec);
        }
        start += 1;
        end -= 1;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_palindrome_true_with_no_removal() {
        assert!(valid_palindrome("radar".to_string()));
    }

    #[test]
    fn test_valid_palindrome_true_with_one_removal() {
        assert!(valid_palindrome("radacr".to_string()));
    }

    #[test]
    fn test_valid_palindrome_false() {
        assert!(!valid_palindrome("random".to_string()));
    }

    #[test]
    fn test_valid_palindrome_empty_string() {
        assert!(valid_palindrome("".to_string()));
    }

    #[test]
    fn test_valid_palindrome_single_character() {
        assert!(valid_palindrome("a".to_string()));
    }

    #[test]
    fn test_valid_palindrome_long_string() {
        let long_palindrome = "a".repeat(100000);
        assert!(valid_palindrome(long_palindrome));
    }
}
