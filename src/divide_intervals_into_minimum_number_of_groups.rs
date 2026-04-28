use std::collections::BinaryHeap;

fn min_groups(mut intervals: Vec<Vec<i32>>) -> i32 {
    intervals.sort_by_key(|v| v[1]);
    let n = intervals.len();
    let mut max_heap = BinaryHeap::new();
    max_heap.push(intervals[n-1][0]);
    for i in (0..n-1).rev() {
        let peek = *max_heap.peek().unwrap();
        if intervals[i][1] <= peek {
            max_heap.pop();
        }
        max_heap.push(intervals[i][0]);
    }

    max_heap.len() as i32
}

pub fn main() {
    let intervals = [[5,10],[6,8],[1,5],[2,3],[1,10]].into_iter().map(Vec::from).collect();
    println!("{}", min_groups(intervals));
}

