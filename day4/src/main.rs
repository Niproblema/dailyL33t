fn main() {
    println!("Hello, world!");
}

// https://leetcode.com/problems/replace-elements-with-greatest-element-on-right-side/submissions/1083741316/
pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
    let mut max = -1;
    let mut out: Vec<i32> = vec![0; arr.len()];
    for i in (0..arr.len()).rev() {
        out[i] = max;
        if (arr[i] > max) {
            max = arr[i];
        }
    }

    out
}

// https://leetcode.com/problems/k-th-symbol-in-grammar/description/?envType=daily-question&envId=2023-10-25
pub fn kth_grammar(n: i32, k: i32) -> i32 {
    let mut neg = false;
    let mut kk = k;
    let mut nn = n as u32;
    while nn > 0 {
        let lv_max = i32::pow(2, nn-1);
        if lv_max >= kk {
            nn -= 1;
        } else {
            kk -= lv_max;
            neg = !neg;
        }
    }
    if neg {1} else {0}
}
