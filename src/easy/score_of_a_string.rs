// https://leetcode.com/problems/score-of-a-string/description/

pub fn score_of_string(s: String) -> i32 {
    let s_as_bytes = s.as_bytes();
    let mut score: i32 = 0;

    for i in 0..(s_as_bytes.len() - 1) {
        score += (s_as_bytes[i] as i32 - s_as_bytes[i + 1] as i32).abs();
    }

    score
}
