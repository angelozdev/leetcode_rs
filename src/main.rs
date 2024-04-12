use leetcode_rs::easy;

fn main() {
    assert_eq!(
        easy::concatenation_of_array::get_concatenation([1, 2, 3].to_vec()),
        [1, 2, 3, 1, 2, 3].to_vec()
    );

    assert_eq!(
        easy::concatenation_of_array::get_concatenation([1, 3, 2, 1].to_vec()),
        [1, 3, 2, 1, 1, 3, 2, 1].to_vec()
    );
}
