use std::{cmp::Reverse, collections::BinaryHeap};

struct KthLargest {
    k: i32,
    min_heap: BinaryHeap<Reverse<i32>>,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut min_heap = BinaryHeap::new();
        for num in &nums {
            min_heap.push(Reverse(*num));
            if min_heap.len() as i32 > k {
                min_heap.pop();
            }
        }

        Self { k, min_heap }
    }

    fn add(&mut self, val: i32) -> i32 {
        self.min_heap.push(Reverse(val));
        if (self.min_heap.len() as i32) > self.k {
            self.min_heap.pop();
        }

        let Reverse(kth) = self.min_heap.peek().unwrap();
        *kth
    }
}

pub fn main() {
    let _kth_largest = KthLargest::new(4, vec![7, 7, 7, 7, 8, 3]);
}

