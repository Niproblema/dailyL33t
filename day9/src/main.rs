fn main() {
    //let inp : Vec<i32> = vec![0,1,2,3,4,5,6,7,8];
    let inp: Vec<i32> = vec![1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1];
    let res = Solution::sort_by_bits(inp);
    println!("{:?}", res);
}

// https://leetcode.com/problems/sort-integers-by-the-number-of-1-bits/
pub struct Solution {}

impl Solution {
    pub fn sort_by_bits(mut arr: Vec<i32>) -> Vec<i32> {
        arr.sort_unstable_by_key(|&x| (x.count_ones(), x));
        arr
    }
}
