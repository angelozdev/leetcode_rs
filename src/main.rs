use leetcode_rs::medium::top_k_frequent_elements::top_k_frequent;

fn main() {
    let v = vec![1, 1, 2, 2, 3, 1, 2, 3, 0, 4, 6, 6, 1, 3];
    let n = top_k_frequent(v, 3);
    println!("{:?}", n)
}
