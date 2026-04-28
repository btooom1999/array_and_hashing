fn remove_covered_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
    intervals.sort_by(|a, b| a[0].cmp(&b[0]).then(b[1].cmp(&a[1])));
    let n = intervals.len();
    let mut res = n;

    let mut current = intervals[0].clone();
    for i in 1..n {
        if intervals[i][0] >= current[0] && intervals[i][1] <= current[1] {
            res -= 1;
        } else {
            current = intervals[i].clone();
        }
    }

    res as i32
}

pub fn main() {
    let intervals = [[1,4],[1,6],[2,3]].into_iter().map(Vec::from).collect();
    println!("{}", remove_covered_intervals(intervals));
}
