use std::{cmp::Reverse, collections::BinaryHeap};

fn assign_tasks(servers: Vec<i32>, tasks: Vec<i32>) -> Vec<i32> {
    let mut min_heap = BinaryHeap::new();

    for (i, server) in servers.into_iter().enumerate() {
        min_heap.push(Reverse((server, i)));
    }

    let mut res = Vec::new();
    let mut queue = BinaryHeap::<Reverse<(i32, i32, usize)>>::new();
    let mut time = 0;
    for i in 0..tasks.len() {
        time = time.max(i as i32);

        if min_heap.is_empty() && let Some(Reverse(data)) = queue.peek() {
            time = data.0;
        }

        while let Some(Reverse(front)) = queue.peek() && time >= front.0 {
            min_heap.push(Reverse((front.1,front.2)));
            queue.pop();
        }

        if let Some(Reverse(candidate)) = min_heap.pop() {
            queue.push(Reverse((time + tasks[i], candidate.0, candidate.1)));
            res.push(candidate.1 as i32);
        }
    }

    res
}

pub fn main() {
    let servers = [5, 5, 9, 9, 9, 5, 10];
    let tasks = [100_000; 100_000];
    println!("{:?}", assign_tasks(servers.into(), tasks.into()));
}
