pub fn ceiling_of_target(arr: &[i32], target: i32) -> Option<i32> {
    let mut low: usize = 0;
    let mut high: usize = arr.len() - 1;

    while low <= high {
        let mid: usize = ((high - low) / 2) + low;

        if arr[mid] >= target {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }

    if low < arr.len() {
        Some(arr[low])
    } else {
        None
    }
}
