fn remove_interval(interval: Vec<Vec<i32>>, to_be_removed: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    let (x, y) = (to_be_removed[0], to_be_removed[1]);
    for pair in interval {
        let (a, b) = (pair[0], pair[1]);
        if a < x {
            res.push(vec![a, b.min(x)]);
        }
        if b > y {
            res.push(vec![a.max(y), b]);
        }
    }

    res
}

pub fn main() {
    let interval = vec![vec![-5,-4], vec![-3,-2], vec![1,2], vec![3,5], vec![8,9]];
    let to_be_removed = vec![-1, 4];
    // let interval = vec![vec![0,5]];
    // let to_be_removed= vec![2,3];
    // let interval = vec![vec![0,2], vec![3,4], vec![5,7]];
    // let to_be_removed = vec![1,6];
    println!("{:?}", remove_interval(interval, to_be_removed));
}
