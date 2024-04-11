// https://leetcode.com/problems/top-k-frequent-elements/description/

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    use std::{
        cmp::Reverse,
        collections::{BinaryHeap, HashMap},
    };

    let mut map: HashMap<i32, usize> = HashMap::new();

    for n in nums {
        *map.entry(n).or_insert(0) += 1;
    }

    let mut heap = BinaryHeap::with_capacity(k as usize);

    for (&num, &count) in map.iter() {
        if heap.len() < k as usize {
            heap.push(Reverse((count, num)));
        } else if count > heap.peek().unwrap().0 .0 {
            heap.pop();
            heap.push(Reverse((count, num)));
        }
    }

    heap.into_iter().map(|x| x.0 .1).collect()
}
