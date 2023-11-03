fn main() {
    let target = vec![1,2];
    let n = 4;
    let res = Solution::build_array(target, n);

    println!("{:?}", res);
}

// https://leetcode.com/problems/build-an-array-with-stack-operations/?envType=daily-question&envId=2023-11-03
pub struct Solution {}
impl Solution {
    const PUSH: &'static str = "Push";
    const POP: &'static str = "Pop";

    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut res: Vec<String> = Vec::with_capacity(n as usize);
        let mut stream_val = 1;

        for el in target {
            while stream_val < el {
                res.push(String::from(Self::PUSH));
                res.push(String::from(Self::POP));

                stream_val += 1;
            }
            res.push(String::from(Self::PUSH));
            stream_val += 1;
        }

        res
    }
}
