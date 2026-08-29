use std::collections::BinaryHeap;

fn max_kelements(nums: Vec<i32>, mut k: i32) -> i64 {
    let mut max_heap = nums.into_iter().collect::<BinaryHeap<_>>();
    let mut res = 0;

    while k > 0 {
        let val = max_heap.pop().unwrap();
        res += val as i64;
        max_heap.push((val as f64 / 3f64).ceil() as i32);
        k -= 1;
    }

    res
}

pub fn main() {
    let nums = [1,10,3,3,3].to_vec();
    let k = 3;
    println!("{}", max_kelements(nums, k));
}
