fn main() {
    let arr = vec![2,1,3,5,4,6,7];
    let k = 2;
    let res = Solution::get_winner(arr, k);
    println!("{:?}",res);
}

// https://leetcode.com/problems/find-the-winner-of-an-array-game/?envType=daily-question&envId=2023-11-05
pub struct Solution {}
impl Solution {
    pub fn get_winner(arr: Vec<i32>, k: i32) -> i32 {
        let mut max = i32::MIN;
        let mut max_index = 0;
        let mut i = 0;
        while i < arr.len() {
            let el = arr[i];
            if el > max {
                max = el;
                max_index = if i == 0 {0} else {i-1};
            } 
            if i - max_index >= k as usize {
                break;
            }
            i+=1;
        }

        max
    }
}
