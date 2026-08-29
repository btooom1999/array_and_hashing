fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
    intervals.sort_by_key(|v| v[1]);

    let mut current = intervals[0].clone();
    let mut count = intervals.len()-1;
    for pair in intervals.into_iter().skip(1) {
        if pair[1] > current[1] && pair[0] >= current[1] {
            current = pair;
            count -= 1;
        }
    }

    count as i32
}

pub fn main() {
    let intervals = [[1,2],[2,3],[3,4],[1,3]].into_iter().map(Vec::from).collect();
    println!("{}", erase_overlap_intervals(intervals));
}
