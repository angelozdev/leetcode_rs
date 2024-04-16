pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
    use std::collections::HashSet;
    let hset: HashSet<char> = jewels.chars().collect();
    let mut ans: i32 = 0;

    for c in stones.chars() {
        if hset.contains(&c) {
            ans += 1;
        }
    }

    ans
}
