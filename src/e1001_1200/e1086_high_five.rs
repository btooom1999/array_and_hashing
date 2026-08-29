use std::{cmp::Reverse, collections::BinaryHeap};

fn high_five(items: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut res = vec![None as Option<(i32, BinaryHeap<Reverse<i32>>)>; 1001];
    for item in items {
        let (id, num) = (item[0] as usize, item[1]);
        if res[id].is_none() {
            res[id] = Some((id as i32, BinaryHeap::new()));
        }

        let heap = res[id].as_mut().unwrap();
        heap.1.push(Reverse(num));
        if heap.1.len() > 5 {
            heap.1.pop();
        }
    }

    res
        .into_iter()
        .flatten()
        .filter(|v| v.1.len() == 5)
        .map(|(id, vec)|
            vec![
                id,
                vec.into_iter().map(|v| v.0).sum::<i32>() / 5
            ]
        )
        .collect()
}

pub fn main() {
    let items = [[1,91], [1,92], [2,93], [2,97], [1,60], [2, 77], [1,65], [1,87], [1,100], [2,100], [2,76]]
        .into_iter()
        .map(Vec::from)
        .collect();
    println!("{:?}", high_five(items));
}
