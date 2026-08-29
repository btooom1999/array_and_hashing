use std::collections::{BinaryHeap, HashMap, VecDeque};

fn reorganize_string(s: String) -> String {
    let mut hashmap = HashMap::new();
    for c in s.chars() {
        *hashmap.entry(c).or_insert(0) += 1;
    }

    let mut queue = VecDeque::new();

    let mut chars = Vec::new();
    let mut max_heap = hashmap.into_iter().map(|(k,v)| (v, k)).collect::<BinaryHeap<_>>();
    while let Some(mut data) = max_heap.pop() {
        if let Some(&last) = chars.last() && last == data.1 {
            return String::new();
        }

        chars.push(data.1);

        if let (Some(last), Some((_, first))) = (chars.last(), queue.front()) && last != first {
            max_heap.push(queue.pop_front().unwrap());
        }

        if data.0 > 1 {
            data.0 -= 1;
            queue.push_back(data);
        }
    }

    if !queue.is_empty() {
        return String::new();
    }

    chars.iter().collect()
}

pub fn main() {
    let s = "aab".to_string();
    println!("{:?}", reorganize_string(s));
}
