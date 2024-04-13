pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
    let mut ans: Vec<i32> = Vec::with_capacity(nums.len());

    for &n in nums.iter() {
        let n = n as usize;
        let value = nums[n as usize];
        ans.push(value);
    }

    ans
}
