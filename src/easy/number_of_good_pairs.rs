// https://leetcode.com/problems/number-of-good-pairs/description/

pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut result: i32 = 0;

    for n in nums.iter() {
        if let Some(&x) = map.get(n) {
            result += x;
        }

        *map.entry(*n).or_insert(0) += 1;
    }

    result
}

// pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
//     let mut map: HashMap<i32, i32> = HashMap::new();

//     for &n in nums.iter() {
//         map.entry(n).and_modify(|x| *x += 1).or_insert(0);
//     }

//     let result: i32 = map.iter().fold(0, |acc, (_, value)| {
//         let sum = (value * (value + 1)) / 2;
//         return acc + sum;
//     });

//     result
// }
