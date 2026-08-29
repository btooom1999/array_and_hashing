use std::collections::BinaryHeap;

fn last_stone_weight(stones: Vec<i32>) -> i32 {
    let mut heap: BinaryHeap<i32> = BinaryHeap::from(stones);
    let mut max = None;

    while let Some(popped) = heap.pop() {
        if let Some(value) = max {
            if value != popped {
                let res: i32 = value - popped;
                heap.push(res.abs());
            }
            max = None;
        } else {
            max = Some(popped);
        }
    }

    max.unwrap_or(0)
}

pub fn main() {
    let stones = [2,7,4,1,8,1];
    println!("{}", last_stone_weight(stones.into()));
}
