pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
    let mut ans: Vec<i32> = vec![0; nums.len()];

    for i in 0..nums.len() {
        ans[i] = nums[nums[i] as usize];
    }

    ans
}
