use std::{cmp::Reverse, collections::{BinaryHeap, HashMap, VecDeque}};

fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
    let mut hashmap = HashMap::new();
    for task in tasks {
        hashmap.entry(task).and_modify(|v| *v += 1).or_insert(1);
    }

    let mut heap = BinaryHeap::from(hashmap.into_values().map(|v| Reverse(-v)).collect::<Vec<_>>());
    let mut queue = VecDeque::new();
    let mut time = 0;
    while !heap.is_empty() || !queue.is_empty() {
        time += 1;

        if let Some(Reverse(data)) = heap.pop() && data != -1 {
            queue.push_back((data+1, time + n));
        }

        if let Some(data) = queue.front() && time == data.1 {
            heap.push(Reverse(queue.pop_front().unwrap().0));
        }
    }

    time
}

pub fn main() {
    let tasks = ['A', 'A', 'A', 'B', 'B', 'B'];
    let n = 2;
    println!("{}", least_interval(tasks.into(), n));
}
