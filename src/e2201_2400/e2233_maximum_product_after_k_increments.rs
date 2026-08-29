use std::{cmp::Reverse, collections::BinaryHeap};

const MOD: i64 = 1_000_000_007;

fn maximum_product(nums: Vec<i32>, k: i32) -> i32 {
    let mut heap = nums.into_iter().map(Reverse).collect::<BinaryHeap<_>>();

    for _ in 0..k {
        heap.peek_mut().unwrap().0 += 1;
    }

    heap.into_iter().fold(1, |res, Reverse(x)| res * (x as i64) % MOD) as i32
}

pub fn main() {
    let nums = [0,4].to_vec();
    let k = 2;
    println!("{}", maximum_product(nums, k));
}
