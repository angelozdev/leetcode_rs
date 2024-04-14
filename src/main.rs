use leetcode_rs::easy;

fn main() {
    assert_eq!(
        easy::defanging_an_ip_address::defang_i_paddr(String::from("1.1.1.1")),
        String::from("1[.]1[.]1[.]1")
    );

    assert_eq!(
        easy::defanging_an_ip_address::defang_i_paddr(String::from("255.100.50.0")),
        String::from("255[.]100[.]50[.]0")
    );
}
