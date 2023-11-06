fn main() {
    println!("Hello, world!");
}

pub struct Solution {}
impl Solution {
    pub fn get_last_moment(n: i32, left: Vec<i32>, right: Vec<i32>) -> i32 {
        let max_distance_left = left.iter().max().unwrap_or(&0);
        let max_distance_right = n- right.iter().min().unwrap_or(&n);
        std::cmp::max(*max_distance_left, max_distance_right)
    }
}
