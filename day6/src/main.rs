use std::str;

fn main() {
    let input = String::from("a");
    println!("{}", longest_palindrome(input));
}

// https://leetcode.com/problems/longest-palindromic-substring/description/?envType=daily-question&envId=2023-10-27
pub fn longest_palindrome(s: String) -> String {
    let mut longest: usize = 0;
    let (mut p1, mut p2): (usize, usize) = (0, 0);

    let sequence = s.as_bytes();

    for i in 1..sequence.len() {
        let mut mid = 1;
        while mid <= i && mid + i < sequence.len() && sequence[i - mid] == sequence[i + mid] {
            let size = mid * 2 + 1;
            if size > longest {
                longest = size;
                p1 = i - mid;
                p2 = i + mid;
            }
            mid += 1;
        }

        mid = 0;
        while mid < i && mid + i < sequence.len() && sequence[i - mid - 1] == sequence[i + mid] {
            let size = mid * 2 + 2;
            if size > longest {
                longest = size;
                p1 = i - mid - 1;
                p2 = i + mid;
            }
            mid += 1;
        }
    }

    return str::from_utf8(&sequence[p1..p2+1]).unwrap().to_owned();
}
