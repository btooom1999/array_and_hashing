use std::{cmp::Reverse, collections::BinaryHeap};

fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
    let k = k as usize;
    let n = nums2.len();
    let mut max_heap = BinaryHeap::new();
    for (i, &num) in nums1.iter().enumerate() {
        max_heap.push(Reverse((num + nums2[0], i, 0)));
    }

    let mut res = Vec::new();
    while res.len() != k {
        let (_, i, j) = max_heap.pop().unwrap().0;
        res.push(vec![nums1[i], nums2[j]]);
        if j+1 != n {
            max_heap.push(Reverse((nums1[i] + nums2[j+1], i, j+1)));
        }
    }

    res
}

pub fn main() {
    let nums1 = [1,7,11].to_vec();
    let nums2 = [2,4,6].to_vec();
    let k = 3;
    println!("{:?}", k_smallest_pairs(nums1, nums2, k));
}
