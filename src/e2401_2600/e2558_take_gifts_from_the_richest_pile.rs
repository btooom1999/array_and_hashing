use std::{collections::BinaryHeap};

fn pick_gifts(gifts: Vec<i32>, mut k: i32) -> i64 {
    let mut heap = BinaryHeap::from(gifts);
    while k > 0 {
        if let Some(popped) = heap.pop() {
            let res = (popped as f64).sqrt() as i32;
            heap.push(res);
            k -= 1;
        }
    }

    heap.into_iter().fold(0, |acc, num| {
        acc + num as i64
    })
}

pub fn main() {
    let gifts = [25,64,9,4,100];
    let k = 4;
    println!("{}", pick_gifts(gifts.into(), k));
}
