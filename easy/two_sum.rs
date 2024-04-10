fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();

    for (index, &n) in nums.iter().enumerate() {
        let index = index as i32;
        let complement: i32 = target - n;

        if let Some(&complement_index) = map.get(&complement) {
            return vec![complement_index, index];
        }

        map.insert(n, index);
    }

    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_straightforward_case() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = two_sum(nums, target);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_no_solution() {
        let nums = vec![1, 2, 3, 4];
        let target = 8;
        let result = two_sum(nums, target);
        assert!(result.is_empty());
    }

    #[test]
    fn test_with_negative_numbers() {
        let nums = vec![-3, 4, 3, 90];
        let target = 0;
        let result = two_sum(nums, target);
        assert_eq!(result, vec![0, 2]);
    }

    #[test]
    fn test_same_number_twice() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let result = two_sum(nums, target);
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn test_larger_set_of_numbers() {
        let nums = vec![10, 15, 3, 7, 11, 2];
        let target = 9;
        let result = two_sum(nums, target);
        assert_eq!(result, vec![3, 5]);
    }
}
