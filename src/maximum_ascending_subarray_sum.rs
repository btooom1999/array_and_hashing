fn max_ascending_sum(nums: Vec<i32>) -> i32 {
    let mut sum = 0;

    for i in 0..nums.len() {
        let (mut var, mut flag, mut temp_sum) = (nums[i], true, nums[i]);

        for val in &nums[i + 1..] {
            if !flag {
                break;
            }

            if *val <= var {
                flag = false;
            }

            if flag {
                var = *val;
                temp_sum += *val;
            }
        }

        sum = std::cmp::max(sum, temp_sum);
    }

    sum
}

pub fn main() {
    let nums = vec![3, 6, 10, 1, 8, 9, 9, 8, 9];
    println!("{}", max_ascending_sum(nums));
}
