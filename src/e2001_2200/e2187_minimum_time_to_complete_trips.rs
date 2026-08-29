fn minimum_time(time: Vec<i32>, total_trips: i32) -> i64 {
    let mut l = 0;
    let mut r = i64::MAX;

    while l <= r {
        let m = (l + r) / 2;
        let mut total_trips = total_trips as i64;
        for t in &time {
            let t = *t as i64;
            total_trips -= m / t;
            if total_trips < 0 {
                break;
            }
        }

        if total_trips <= 0 {
            r = m - 1;
        } else {
            l = m + 1;
        }
    }

    l
}

pub fn main() {
    let time = [10000].to_vec();
    let total_trips = 10000000;
    println!("{}", minimum_time(time, total_trips));
}
