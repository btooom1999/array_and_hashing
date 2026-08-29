use std::collections::BinaryHeap;

fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
    let mut max_heap = BinaryHeap::new();
    for (i, &score) in score.iter().enumerate() {
        max_heap.push((score, i));
    }

    let mut i = 1;
    let mut res = vec![String::new(); score.len()];
    while let Some(data) = max_heap.pop() {
        res[data.1] = match i {
            1 => "Gold Medal".to_string(),
            2 => "Silver Medal".to_string(),
            3 => "Bronze Medal".to_string(),
            _ => i.to_string(),
        };
        i += 1;
    }

    res
}

pub fn main() {
    let score = [10,3,8,9,4];
    println!("{:?}", find_relative_ranks(score.into()));
}
