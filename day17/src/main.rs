use std::{collections::BinaryHeap, cmp::Reverse};

fn main() {
    let dist = vec![1,3,4];
    let speed = vec![1,1,1];
    let res = Solution::eliminate_maximum(dist, speed);
    println!("{:?}", res);
}


// https://leetcode.com/problems/eliminate-maximum-number-of-monsters/description/?envType=daily-question&envId=2023-11-07
pub struct Solution{}
impl Solution {
    pub fn eliminate_maximum(dist: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut priority : BinaryHeap<Reverse<i32>> = BinaryHeap::new(); 

        let mut i = 0;
        while i < dist.len(){
            let distance = dist[i];
            let speed = speed[i];
            let time_left = distance/speed + (distance%speed != 0) as i32;
            priority.push(Reverse(time_left));
            i+=1;
        }

        let mut count = 0;
        while let Some(Reverse(val)) = priority.pop(){
            //let time_spent = count;
            if val <= count {
                break;
            }
            count+=1;
        }

        count
    }
}