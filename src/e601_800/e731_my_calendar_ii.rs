struct MyCalendarTwo {
    overlaps: Vec<(i32, i32)>,
    schedules: Vec<(i32, i32)>,
}

impl MyCalendarTwo {
    fn new() -> Self {
        Self { overlaps: Vec::new(), schedules: Vec::new() }
    }

    fn book(&mut self, start_time: i32, end_time: i32) -> bool {
        for &(x, y) in &self.overlaps {
            if start_time < y && end_time > x {
                return false;
            }
        }
        for &(x, y) in &self.schedules {
            if start_time < y && end_time > x {
                self.overlaps.push((start_time.max(x), end_time.min(y)));
            }
        }

        self.schedules.push((start_time, end_time));
        true
    }
}

pub fn main() {
    let mut my_calendar_two = MyCalendarTwo::new();
    println!("{}", my_calendar_two.book(10,11));
    println!("{}", my_calendar_two.book(50,60));
    println!("{}", my_calendar_two.book(10,11));
    // println!("{}", my_calendar_two.book(5,15));
    // println!("{}", my_calendar_two.book(5,10));
}
