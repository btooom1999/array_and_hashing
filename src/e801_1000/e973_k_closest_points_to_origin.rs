use std::collections::BinaryHeap;

fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let mut heap = BinaryHeap::new();
    for (i, point) in points.iter().enumerate() {
        let (x, y) = (point[0], point[1]);
        let res = (x*x + y*y) as i64;
        heap.push((res, i));
        if heap.len() > k as usize {
            heap.pop();
        }
    }

    heap.into_iter().map(|x| points[x.1].clone()).collect()
}

pub fn main() {
    let points = [[6,10],[-3,3],[-2,5],[0,2]].into_iter().map(|vec| vec.to_vec()).collect::<Vec<_>>();
    let k = 1;
    println!("{:?}", k_closest(points, k));
}
