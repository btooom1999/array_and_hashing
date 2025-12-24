fn pivot_index(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return 0;
    }

    let mut left_sum = 0;
    let sum: i32 = nums.iter().sum();
    for (i, val) in nums.iter().enumerate() {
        let right_sum = sum - val - left_sum;

        if left_sum == right_sum {
            return i as i32;
        }

        left_sum += val;
    }

    -1
}

pub fn main() {
    let nums = vec![1, 7, 3, 6, 5, 6];
    println!("{}", pivot_index(nums));
}
