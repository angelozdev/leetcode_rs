// https://leetcode.com/problems/defanging-an-ip-address/description/

pub fn defang_i_paddr(address: String) -> String {
    address.replace(".", "[.]")
}
