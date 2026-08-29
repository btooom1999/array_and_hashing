struct MyCalendar(Vec<(i32, i32)>);

impl MyCalendar {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn book(&mut self, start_time: i32, end_time: i32) -> bool {
        let i = self.0.partition_point(|v| v.0 <= start_time);
        if i > 0 && self.0[i-1].1 > start_time {
            return false;
        }

        if i < self.0.len() && self.0[i].0 < end_time {
            return false;
        }

        self.0.insert(i, (start_time, end_time));
        true
    }
}
