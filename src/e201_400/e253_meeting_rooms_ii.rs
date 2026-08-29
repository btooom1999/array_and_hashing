use std::collections::VecDeque;

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

fn min_meeting_rooms(mut intervals: Vec<Interval>) -> i32 {
    intervals.sort_by_key(|v| v.end);

    let mut rooms = VecDeque::<Interval>::new();
    for interval in intervals.into_iter() {
        if let Some(i) = rooms.iter().position(|v| interval.start >= v.end) {
            rooms[i] = interval;
        } else {
            rooms.push_front(interval);
        }
    }

    rooms.len() as i32
}

pub fn main() {
    let intervals = [(25,579),(218,918),(1281,1307),(623,1320),(685,1353),(1308,1358)].into_iter().map(Interval::new).collect();
    println!("{:?}", min_meeting_rooms(intervals));
}
