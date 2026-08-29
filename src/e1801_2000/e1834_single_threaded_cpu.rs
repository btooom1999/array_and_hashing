use std::{cmp::Reverse, collections::BinaryHeap};

fn get_order(mut tasks: Vec<Vec<i32>>) -> Vec<i32> {
    for (i, task) in tasks.iter_mut().enumerate() {
        task.push(i as i32);
    }

    tasks.sort_by(|a, b| a[0].cmp(&b[0]));

    let mut min_heap = BinaryHeap::new();
    let mut res = Vec::new();
    let mut finished_time = 1;
    let mut i = 0;
    let n = tasks.len();
    while res.len() != n {
        while i < n && tasks[i][0] <= finished_time {
            min_heap.push(Reverse((tasks[i][1], tasks[i][2])));
            i += 1;
        }

        if i < n && min_heap.is_empty() {
            finished_time = tasks[i][0];
        }

        if let Some(Reverse(data)) = min_heap.pop() {
            res.push(data.1);
            finished_time += data.0;
        }
    }

    res
}

pub fn main() {
    let tasks = [[1,2],[2,4],[3,2],[4,1]].into_iter().map(Vec::from).collect::<Vec<_>>();
    println!("{:?}", get_order(tasks));
}
