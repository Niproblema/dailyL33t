// https://leetcode.com/problems/minimum-size-subarray-sum/description/

fn main() {
    println!("{}", min_sub_array_len(7, vec![1, 1, 1, 1, 7]));
}

pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    let size = nums.len();

    let mut best = u32::MAX;
    let (mut p1, mut p2) = (0, 0);
    let mut window_sum = nums.first().map_or(0, |x| *x);

    loop {
        // Evalute the new window against the best
        let window_width = (p2 - p1 + 1) as u32;
        if window_sum >= target && window_width < best {
            best = window_width;
        }

        // If window size is too small, try widening the window
        if window_sum < target && p2 < size - 1 {
            p2 += 1;
            window_sum += nums[p2];
        }
        // If window size is greater than required, try narrowing the window
        else if window_sum >= target && p1 < p2 {
            window_sum -= nums[p1];
            p1 += 1;
        }
        // If stuck, break.
        else {
            break;
        }
    }

    if best == u32::MAX {
        0
    } else {
        best as i32
    }
}
