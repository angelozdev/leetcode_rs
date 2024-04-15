use leetcode_rs::easy::final_value_of_variable_after_performing_operations::final_value_after_operations;

fn main() {
    assert_eq!(
        final_value_after_operations(
            ["--X".to_string(), "X++".to_string(), "X++".to_string()].to_vec()
        ),
        1
    );
    assert_eq!(
        final_value_after_operations(
            ["++X".to_string(), "++X".to_string(), "X++".to_string()].to_vec()
        ),
        3
    );
    assert_eq!(
        final_value_after_operations(
            [
                "X++".to_string(),
                "++X".to_string(),
                "--X".to_string(),
                "X--".to_string()
            ]
            .to_vec()
        ),
        0
    );
}
