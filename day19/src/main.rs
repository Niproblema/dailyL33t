fn main() {
    println!("Hello, world!");
}

// https://leetcode.com/problems/count-number-of-homogenous-substrings/?envType=daily-question&envId=2023-11-09
pub struct Solution {}
impl Solution {
    const MOD: usize = 1_000_000_007;

    pub fn count_homogenous(s: String) -> i32 {
        let sequence = s.as_bytes();
        let len = sequence.len();
        let (mut p1, mut p2): (usize, usize) = (0, 0);
        let mut count : i32= 0;

        while p2 < len{
            if p1 < len && sequence[p1] == sequence[p2]{
                p1+=1;
            }else{
                let seq_len = p1-p2;
                let seq_count = seq_len*(seq_len+1)/2;
                count+= (seq_count % Self::MOD) as i32;
                p2 = p1;
            }
        }
        count
    }
}