fn min_start_value(mut nums: Vec<i32>) -> i32 {
    let mut start_value = nums.iter().min().unwrap_or(&0).to_owned();
    if start_value >= 1 {
        return 1;
    }

    start_value = start_value.abs() + 1;
    let mut min = i32::MAX;
    let mut sum = start_value;
    for num in nums.iter_mut() {
        sum += *num;
        *num = sum;
        min = min.min(*num);
    }

    if min < 1 {
        return 1.max(start_value + 1 - min);
    }

    1.max(start_value - min - 1)
}

pub fn main() {
    let nums = vec![2, 3, 5, -5, -1];
    println!("{}", min_start_value(nums));
}
