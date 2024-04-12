// https://leetcode.com/problems/valid-parentheses/description/

pub fn is_valid(s: String) -> bool {
    use std::collections::HashMap;

    let mut stack: Vec<u8> = Vec::new();
    let map: HashMap<u8, u8> = [(b'}', b'{'), (b']', b'['), (b')', b'(')]
        .into_iter()
        .collect();

    for &ch in s.as_bytes() {
        match ch {
            b'(' | b'{' | b'[' => stack.push(ch),
            b')' | b']' | b'}' => {
                let last_char = stack.pop();
                if last_char != Some(*map.get(&ch).unwrap()) {
                    return false;
                }
            }
            _ => return false,
        }
    }

    stack.is_empty()
}
