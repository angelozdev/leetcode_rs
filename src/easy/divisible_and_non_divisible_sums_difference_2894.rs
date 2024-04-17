// pub fn difference_of_sums(n: i32, m: i32) -> i32 {
//     let mut ans: i32 = 0;

//     for i in 1..=n {
//         if i % m == 0 {
//             ans -= i;
//         } else {
//             ans += i;
//         }
//     }

//     ans
// }

fn factorial_sum(n: i32) -> i32 {
    (n * (n + 1)) / 2
}

pub fn difference_of_sums(n: i32, m: i32) -> i32 {
    let k: i32 = n / m;
    let total_sum: i32 = factorial_sum(n);
    let num2: i32 = m * factorial_sum(k);
    let num1: i32 = total_sum - num2;

    num1 - num2
}
