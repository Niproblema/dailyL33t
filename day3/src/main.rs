// https://leetcode.com/problems/grumpy-bookstore-owner/

fn main() {
    let customers: Vec<i32> = vec![10, 1, 7];
    let grumpy: Vec<i32> = vec![0, 0, 0];
    let minutes = 2;

    println!("{}", max_satisfied(customers, grumpy, minutes));
}

pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
    let baseline: i32 = customers
        .iter()
        .enumerate()
        .filter(|x| grumpy[x.0] == 0)
        .fold(0, |s, (_, el)| s + el);

    let (mut p1, mut p2): (usize, usize) = (0, (minutes - 1) as usize);

    // Initialize initial value of the window
    let mut best_window = (0..minutes as usize).fold(0, |acc, num| {
        acc + if grumpy[num] == 0 { 0 } else { customers[num] }
    });

    let mut window = best_window;
    while p2 < customers.len() - 1 {
        if grumpy[p1] == 1 {
            window -= customers[p1];
        }

        p1 += 1;
        p2 += 1;

        if grumpy[p2] == 1 {
            window += customers[p2];
        }

        if window > best_window {
            best_window = window;
        }
    }

    baseline + best_window
}
