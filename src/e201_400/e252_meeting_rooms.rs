#[derive(Debug, Clone)]
pub struct Interval {
    pub start: i32,
    pub end: i32,
}

impl Interval {
    pub fn new(time: (i32, i32)) -> Self {
        Interval { start: time.0, end: time.1 }
    }
}

fn can_attend_meetings(mut intervals: Vec<Interval>) -> bool {
    intervals.sort_by_key(|v| v.start);

    for i in 1..intervals.len() {
        if intervals[i].start < intervals[i-1].end {
            return false;
        }
    }

    true
}

pub fn main() {
    let intervals = [(0,30),(5,10),(15,20)].into_iter().map(Interval::new).collect();
    println!("{:?}", can_attend_meetings(intervals));
}
