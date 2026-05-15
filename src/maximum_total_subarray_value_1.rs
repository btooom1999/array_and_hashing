fn max_total_value(nums: Vec<i32>, k: i32) -> i64 {
    let (mut max, mut min) = (0, 1_000_000_001);
    for num in nums {
        max = max.max(num as i64);
        min = min.min(num as i64);
    }

    k as i64 * (max - min)
}

pub fn main() {
    let nums = [1,3,2].to_vec();
    let k = 2;
    println!("{}", max_total_value(nums, k));
}
