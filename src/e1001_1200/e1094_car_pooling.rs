use std::{cmp::Reverse, collections::BinaryHeap};

fn car_pooling(mut trips: Vec<Vec<i32>>, mut capacity: i32) -> bool {
    trips.sort_by(|a, b| a[1].cmp(&b[1]));

    let mut min_heap = BinaryHeap::new();
    let mut from = 1;
    let mut i = 0;
    let n = trips.len();
    while i < n {
        while from < trips[i][1] && let Some(Reverse((to, amount))) = min_heap.peek() && *to <= trips[i][1] {
            capacity += amount;
            min_heap.pop();
        }

        from = trips[i][1];

        while i < n && trips[i][1] == from {
            capacity -= trips[i][0];
            min_heap.push(Reverse((trips[i][2], trips[i][0])));
            i += 1;
        }

        if capacity < 0 {
            return false;
        }
    }

    true
}

pub fn main() {
    let trips = [[2,1,5],[3,5,7]].into_iter().map(Vec::from).collect();
    let capacity = 4;
    println!("{:?}", car_pooling(trips, capacity));
}
