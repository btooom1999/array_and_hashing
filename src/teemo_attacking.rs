fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
    let mut res = 0;
    let mut during = 0;
    let mut i = 0;
    for (j, &time) in time_series.iter().enumerate() {
        if time > during {
            res += (during - time_series[i]).max(0);
            i = j;
        }

        during = time + duration;
    }

    res + (during - time_series[i])
}

pub fn main() {
    let time_series = [1,4].to_vec();
    let duration = 2;
    println!("{}", find_poisoned_duration(time_series, duration));
}
