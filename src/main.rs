use leetcode_rs::easy;

fn main() {
    assert_eq!(
        easy::build_array_from_permutation::build_array(vec![0, 2, 1, 5, 3, 4]),
        vec![0, 1, 2, 4, 5, 3]
    );
}
