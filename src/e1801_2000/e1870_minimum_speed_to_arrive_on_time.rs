fn min_speed_on_time(dist: Vec<i32>, hour: f64) -> i32 {
    let mut l = 1;
    let mut r = 10_000_000;

    let mut res = -1;

    while l <= r {
        let m = (l + r) / 2;
        let mut total_hours = 0_f64;
        for &d in &dist {
            if total_hours > hour {
                break;
            }

            total_hours = total_hours.ceil() + d as f64 / m as f64;
        }

        if total_hours <= hour {
            res = m;
            r = m - 1;
        } else {
            l = m + 1;
        }
    }

    res
}

pub fn main() {
    let dist = [1,1,100_000].to_vec();
    let hour = 2.01_f64;
    println!("{}", min_speed_on_time(dist, hour));
}
