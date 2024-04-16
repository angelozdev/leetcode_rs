use leetcode_rs::easy::jewels_and_stones_771::num_jewels_in_stones;

fn main() {
    assert_eq!(
        num_jewels_in_stones("aA".to_string(), "aAAbbbb".to_string()),
        3
    );

    assert_eq!(num_jewels_in_stones("z".to_string(), "ZZ".to_string()), 0);
}
