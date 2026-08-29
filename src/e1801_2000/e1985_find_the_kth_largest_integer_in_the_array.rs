use std::collections::BinaryHeap;

fn kth_largest_number(nums: Vec<String>, mut k: i32) -> String {
    let mut max_heap = BinaryHeap::new();
    for num in nums {
        max_heap.push((num.len(), num));
    }

    while k > 1 {
        max_heap.pop();
        k -= 1;
    }

    max_heap.pop().map_or(String::new(), |v| v.1)
}

pub fn main() {
    let nums = ["2", "21", "12", "1"].into_iter().map(String::from).collect::<Vec<_>>();
    let k = 3;
    println!("{:?}", kth_largest_number(nums, k));
}
