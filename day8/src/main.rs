fn main() {
    let bucks :i32 = 4;
    let m_die : i32 = 15;
    let m_test :i32 = 30;
    let res :i32 = Solution::poor_pigs(bucks, m_die, m_test);
    println!("{}", res);
}

// https://leetcode.com/problems/poor-pigs/description/?envType=daily-question&envId=2023-10-29
pub struct Solution {}
impl Solution {
    pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
        let res = 0;
        let times = minutes_to_test/minutes_to_die;
        for i in 1..buckets {
            if (times+1).pow(i as u32) >= buckets{
                return i;
            }
        }
        return 0;
    }  
