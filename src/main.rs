use leetcode_rs::easy;

fn main() {
    assert_eq!(
        easy::number_of_good_pairs::num_identical_pairs(vec![1, 2, 3, 1, 1, 3]),
        4
    );
    assert_eq!(
        easy::number_of_good_pairs::num_identical_pairs(vec![1, 1, 1, 1]),
        6
    );
    assert_eq!(
        easy::number_of_good_pairs::num_identical_pairs(vec![1, 2, 3]),
        0
    );
}
