pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
    let mut x: i32 = 0;

    for op in operations.iter() {
        match op.as_str() {
            "++X" | "X++" => x += 1,
            "--X" | "X--" => x -= 1,
            _ => panic!("Something happend"),
        }
    }

    x
}

// pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
//     let mut x: i32 = 0;

//     for op in operations.iter() {
//         let sign = op.chars().nth(1).unwrap();
//         x += if sign == '+' { 1 } else { -1 };
//     }

//     x
// }
