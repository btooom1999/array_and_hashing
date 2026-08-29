use std::collections::BinaryHeap;

fn find_kth_largest(nums: Vec<i32>, mut k: i32) -> i32 {
    let mut binary_heap = BinaryHeap::from(nums);

    while k > 1 {
        binary_heap.pop();
        k -= 1;
    }

    binary_heap.pop().unwrap_or(-1)
}

pub fn main() {
    let nums = [3,2,1,5,6,4];
    let k = 2;
    println!("{}", find_kth_largest(nums.into(), k));
}
