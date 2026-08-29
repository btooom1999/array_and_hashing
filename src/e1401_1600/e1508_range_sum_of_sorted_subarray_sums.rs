use std::collections::BinaryHeap;

const MOD: i32 = 1_000_000_007;

fn range_sum(nums: Vec<i32>, _n: i32, left: i32, mut right: i32) -> i32 {
    let mut max_heap = BinaryHeap::new();
    for i in 0..nums.len() {
        let mut sum = 0;
        for j in i..nums.len() {
            sum = (sum + nums[j]) % MOD;
            max_heap.push(sum);
            if max_heap.len() as i32 > right {
                max_heap.pop();
            }
        }
    }

    let mut sum = 0;
    while right >= left && let Some(num) = max_heap.pop() {
        right -= 1;
        sum = (sum + num) % MOD;
    }

    sum
}

pub fn main() {
    let nums = [1,2,3,4];
    let n = 4;
    let left = 1;
    let right = 5;
    println!("{}", range_sum(nums.into(), n, left, right));
}
