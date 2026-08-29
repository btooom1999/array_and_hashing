use std::collections::BinaryHeap;

fn max_sum_distinct_triplet(x: Vec<i32>, y: Vec<i32>) -> i32 {
    let mut max_heap = BinaryHeap::new();
    for i in 0..y.len() {
        max_heap.push((y[i], x[i]));
    }

    let (mut i, mut j, k) = ((-1, -1), (-1, -1), (-1, -1));
    while let Some(top) = max_heap.pop() {
        if i.1 == top.1 || j.1 == top.1 || k.1 == top.1 {
            continue;
        } else if i.1 == -1 {
            i = top;
        } else if j.1 == -1 {
            j = top;
        } else if k.1 == -1 {
            return i.0 + j.0 + top.0;
        }
    }

    -1
}

pub fn main() {
    let x = [1,2,1,3,2].to_vec();
    let y = [5,3,4,6,2].to_vec();
    println!("{}", max_sum_distinct_triplet(x, y));
}
