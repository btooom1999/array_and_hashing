fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    intervals.sort();

    let mut res = Vec::<Vec<i32>>::new();
    for pair in intervals {
        let (a, b) = (pair[0], pair[1]);
        if let Some(last) = res.last_mut() && last[1] >= a {
            last[1] = last[1].max(b);
        } else {
            res.push(pair);
        }
    }

    res
}

pub fn main() {
    let intervals = [[1,3],[2,6],[8,10],[15,18]].into_iter().map(Vec::from).collect();
    println!("{:?}", merge(intervals));
}
