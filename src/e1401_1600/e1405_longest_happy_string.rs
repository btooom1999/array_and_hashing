use std::{char, collections::{BinaryHeap, HashMap, VecDeque}};

fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
    let mut max_heap = BinaryHeap::new();
    if a > 0 { max_heap.push((a, 'a')); }
    if b > 0 { max_heap.push((b, 'b')); }
    if c > 0 { max_heap.push((c, 'c')); }

    let mut res = Vec::new();
    let mut hashmap = HashMap::<char, VecDeque<usize>>::new();
    let mut queue = VecDeque::new();
    let mut i = 0;
    while let Some(mut data) = max_heap.pop() {
        data.0 -= 1;
        res.push(data.1);
        hashmap.entry(data.1).or_default().push_back(i);

        if data.0 > 0 {
            queue.push_back(data);
        }

        if let Some(&(_, char)) = queue.front() && char != data.1 {
            max_heap.push(queue.pop_front().unwrap());
        }

        i += 1;
    }

    let mut res = res.iter().collect::<String>();
    let mut step = 0;
    if let Some(mut data) = queue.pop_front() {
        let mut indexes = hashmap.remove(&data.1).unwrap();
        while data.0 > 0 && let Some(idx) = indexes.pop_front() {
            res.insert(idx+1 + step, data.1);
            step += 1;
            data.0 -= 1;
        }
    }

    res
}

pub fn main() {
    let a = 1;
    let b = 1;
    let c = 7;
    println!("{}", longest_diverse_string(a, b, c));
}
