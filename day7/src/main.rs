use std::collections::HashMap;

fn main() {
    let res = Solution::count_vowel_permutation(1000);
    println!("{}", res);
}

// https://leetcode.com/problems/count-vowels-permutation/?envType=daily-question&envId=2023-10-28
pub struct Solution {}
impl Solution {
    const SEQ: [&'static [usize]; 5] = [&[1], &[0, 2], &[0, 1, 3, 4], &[2, 4], &[0]];
    const MODULO: u64 = u64::pow(10, 9) + 7_u64;

    pub fn count_vowel_permutation(n: i32) -> i32 {
        // Map of results [char index][depth]
        let mut inter_res: [HashMap<i32, u64>; 5] = [
            HashMap::from([(0, 1)]),
            HashMap::from([(0, 1)]),
            HashMap::from([(0, 1)]),
            HashMap::from([(0, 1)]),
            HashMap::from([(0, 1)]),
        ];

        let mut sum: u64 = 0;
        for i in 0..5 {
            sum += Solution::find_permutations(n - 1, i, &mut inter_res);
        }

        (sum % Solution::MODULO) as i32
    }

    fn find_permutations(n: i32, ch: usize, maps: &mut [HashMap<i32, u64>]) -> u64 {
        if let Some(cached_result) = maps[ch].get(&n) {
            return *cached_result;
        }

        let mut sum = 0;
        for i in Solution::SEQ[ch] {
            sum += Solution::find_permutations(n - 1, *i, maps);
        }

        sum %= Solution::MODULO;
        
        maps[ch].insert(n, sum);

        sum
    }
}
