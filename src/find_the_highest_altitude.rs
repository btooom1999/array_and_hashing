fn largest_altitude(gain: Vec<i32>) -> i32 {
    let mut sum = 0;
    let mut max = 0;
    for num in &gain {
        sum += num;
        max = std::cmp::max(max, sum);
    }

    max
}

pub fn main() {
    let gain = vec![-5, 1, 5, 0, -7];
    println!("{}", largest_altitude(gain));
}
