pub fn min_partitions(n: String) -> i32 {
    let mut ans: u8 = b'0';

    for &b in n.as_bytes() {
        if b == b'9' {
            return 9;
        }

        if b > ans {
            ans = b
        }
    }

    (ans - b'0') as i32
}
