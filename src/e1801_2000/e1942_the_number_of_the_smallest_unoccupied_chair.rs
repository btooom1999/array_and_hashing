use std::{cmp::Reverse, collections::BinaryHeap};

fn smallest_chair(times: Vec<Vec<i32>>, target_friend: i32) -> i32 {
    let mut times = times.into_iter().enumerate().map(|v| (v.0 as i32, v.1[0], v.1[1])).collect::<Vec<_>>();
    times.sort_by(|a,b| b.1.cmp(&a.1));

    let mut chairs = BinaryHeap::new();
    let mut min_heap = BinaryHeap::new();
    let mut chair = 0;
    min_heap.push(Reverse((-1, 0)));

    while let Some((friend, arrival, leaving)) = times.pop() {
        while let Some(&Reverse((time, chair_num))) = min_heap.peek() && time <= arrival {
            min_heap.pop();
            chairs.push(Reverse(chair_num));
        }

        let chair_num = chairs.pop().map_or_else(|| {
            chair += 1;
            chair
        }, |v| v.0);

        if friend == target_friend {
            return chair_num;
        }

        min_heap.push(Reverse((leaving, chair_num)));
    }

    unreachable!()
}

pub fn main() {
    // let times = [[1,4], [2,3],[4,6]].into_iter().map(Vec::from).collect();
    // let target_friend = 1;
    let times = [[33,35],[26,29],[9,28],[4,31],[8,10],[32,34],[15,24],[27,39],[14,36],[1,14],[25,39],[5,27],[6,15],[2,38],[19,36],[24,34],[3,26]].into_iter().map(Vec::from).collect();
    let target_friend = 0;
    println!("{}", smallest_chair(times, target_friend));
}
