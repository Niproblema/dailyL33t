fn main() {
    println!("{}", Solution::is_reachable_at_time(1, 2, 1, 2, 1));
}

// https://leetcode.com/problems/determine-if-a-cell-is-reachable-at-a-given-time/description/?envType=daily-question&envId=2023-11-08
pub struct Solution {}
impl Solution {
    pub fn is_reachable_at_time(sx: i32, sy: i32, fx: i32, fy: i32, t: i32) -> bool {
        let horizontal_diff = (sx - fx).abs();
        let vertical_diff = (sy - fy).abs();
        let sideways = horizontal_diff.min(vertical_diff);

        let shortest_path = horizontal_diff + vertical_diff - sideways;

        if shortest_path == 0 {
            return t != 1;
        }

        shortest_path <= t

        // if t == shortest_path {
        //     return true;
        // }

        // if t - shortest_path % 2 == 0 {
        //     return true;
        // }

        // if sideways > 1 && (t - shortest_path + 1) % 2 == 0 {
        //     return true;
        // }

        // false
    }
}
