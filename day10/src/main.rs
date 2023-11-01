fn main() {
    let inp = vec![5,2,0,3,1];
    let out = Solution::find_array(inp);
    println!("{:?}", out);
}

// https://leetcode.com/problems/find-the-original-array-of-prefix-xor/?envType=daily-question&envId=2023-10-31
pub struct Solution {}
impl Solution {
    pub fn find_array(pref: Vec<i32>) -> Vec<i32> {
        let mut arr = Vec::with_capacity(pref.len());
        arr.push(pref[0]);
        for i in 1..pref.len() {
            arr.push(pref[i] ^ pref[i-1]);
        }
        arr
    }
}