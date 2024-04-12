use leetcode_rs::easy;

fn main() {
    assert!(easy::valid_parentheses::is_valid("()".to_string()));
    assert!(easy::valid_parentheses::is_valid("()[]{}".to_string()));
    assert!(!easy::valid_parentheses::is_valid("(]".to_string()));
    assert!(!easy::valid_parentheses::is_valid("([)]".to_string()));
}
