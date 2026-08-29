use std::{cmp::Reverse, collections::BinaryHeap};

fn get_final_state(nums: Vec<i32>, mut k: i32, multiplier: i32) -> Vec<i32> {
    let mut heap = BinaryHeap::new();
    let n = nums.len();
    for (i, num) in nums.into_iter().enumerate() {
        heap.push(Reverse((num, i)));
    }

    while k > 0 {
        if let Some(Reverse(mut num)) = heap.pop() {
            num.0 *= multiplier;
            heap.push(Reverse(num));
        }
        k -= 1;
    }

    let mut vec = vec![-1; n];
    while let Some(Reverse(data)) = heap.pop() {
        vec[data.1] = data.0;
    }

    vec
}

pub fn main() {
    let nums = [2,1,3,5,6];
    let k = 5;
    let multiplier = 2;
    println!("{:?}", get_final_state(nums.into(), k, multiplier));
}
