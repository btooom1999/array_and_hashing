use std::{cmp::Reverse, collections::BinaryHeap};

fn furthest_building(heights: Vec<i32>, mut bricks: i32, mut ladders: i32) -> i32 {
    let mut max_heap = BinaryHeap::new();
    for i in 0..heights.len()-1 {
        if heights[i] >= heights[i+1] {
            continue;
        }

        let diff = heights[i+1] - heights[i];
        bricks -= diff;
        max_heap.push(Reverse(-diff));

        if bricks < 0 {
            if ladders == 0 {
                return i as _;
            }

            ladders -= 1;
            bricks += -max_heap.pop().unwrap().0;
        }
    }

    heights.len() as i32 - 1
}

pub fn main() {
    let heights = [4,12,2,7,3,18,20,3,19];
    let bricks = 10;
    let ladders = 2;
    println!("{}", furthest_building(heights.into(), bricks, ladders));
}
