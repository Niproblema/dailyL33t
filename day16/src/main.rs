use std::{cmp::Reverse, collections::BinaryHeap};

fn main() {
    let mut obj = SeatManager::new(20);
    let ret_1: i32 = obj.reserve();
    obj.unreserve(0);

    println!("Hello, world!");
}

// https://leetcode.com/problems/seat-reservation-manager/?envType=daily-question&envId=2023-11-06
pub struct SeatManager {
    heap: BinaryHeap<Reverse<i32>>,
    heap_size: i32,
}
impl SeatManager {
    fn new(_n: i32) -> Self {
        SeatManager {
            heap: BinaryHeap::new(),
            heap_size: 1,
        }
    }

    fn reserve(&mut self) -> i32 {
        if let Some(Reverse(item)) = self.heap.pop() {
            item
        } else {
            let rtn = self.heap_size;
            self.heap_size += 1;
            rtn
        }
    }

    fn unreserve(&mut self, seat_number: i32) {
        self.heap.push(Reverse(seat_number))
    }
}
