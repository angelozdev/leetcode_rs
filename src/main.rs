use leetcode_rs::easy::partitioning_into_minimum_number_of_deci_binary_numbers::min_partitions;

fn main() {
    assert_eq!(min_partitions("32".to_string()), 3);
    assert_eq!(min_partitions("823734".to_string()), 8);
    assert_eq!(min_partitions("384276378467769".to_string()), 9);
}
