use std::{cmp::Reverse, collections::BinaryHeap};

fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
    let mut data = nums1.into_iter().zip(nums2).collect::<Vec<_>>();
    data.sort_by(|a, b| b.1.cmp(&a.1));

    let mut min_heap = BinaryHeap::new();
    let mut res = 0;
    let mut sum = 0;
    for (num1, num2) in data {
        sum += num1 as i64;
        min_heap.push(Reverse(num1));

        if min_heap.len() > k as usize && let Some(Reverse(num)) = min_heap.pop() {
            sum -= num as i64;
        }

        if min_heap.len() == k as usize {
            res = res.max(sum * num2 as i64);
        }
    }

    res
}

pub fn main() {
    let nums1 = [1,3,3,2];
    let nums2 = [2,1,3,4];
    let k = 3;
    println!("{:?}", max_score(nums1.into(), nums2.into(), k));
}
