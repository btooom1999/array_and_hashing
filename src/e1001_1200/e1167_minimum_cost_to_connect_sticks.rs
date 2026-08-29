use std::{cmp::Reverse, collections::BinaryHeap};

fn connect_sticks(sticks: Vec<i32>) -> i32 {
    let mut min_heap = sticks.into_iter().map(Reverse).collect::<BinaryHeap<_>>();
    let mut sum = 0;
    while let (Some(a), Some(b)) = (min_heap.pop(), min_heap.pop()) {
        let res = a.0 + b.0;
        sum += res;
        min_heap.push(Reverse(res));
    }

    sum
}

pub fn main() {
    let sticks = [1,8,3,5];
    println!("{:?}", connect_sticks(sticks.into()));
}
