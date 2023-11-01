//https://leetcode.com/problems/remove-element/submissions/1081474324/?envType=study-plan-v2&envId=top-interview-150

fn main() {
    println!("Hello, world!");
}

pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut len = nums.len();

    let mut k = 0;
    while k < len {
        let el = nums[k];
        if el == val {
            nums[k] = nums[len - 1];
            len -= 1;
        } else {
            k += 1;
        }
    }

    k as i32
}
