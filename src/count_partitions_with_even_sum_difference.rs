fn count_partitions(nums: Vec<i32>) -> i32 {
    let mut sum = nums.iter().sum::<i32>();
    let mut prefix = 0;
    let mut res = 0;
    for num in nums.iter().take(nums.len() - 1) {
        sum -= *num;
        prefix += *num;
        if (prefix - sum) % 2 == 0 {
            res += 1;
        }
    }

    res
}

pub fn main() {
    let nums = vec![10, 10, 3, 7, 6];
    println!("{}", count_partitions(nums));
}
