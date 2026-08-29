fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
    let mut res = 1;

    for i in 0..nums.len() {
        if res > nums.len() - i {
            break;
        }

        let (mut increasing_var, mut increasing_flag, mut increasing_num) = (nums[i], true, 1);
        let (mut decreasing_var, mut decreasing_flag, mut decreasing_num) = (nums[i], true, 1);

        for val in &nums[i + 1..] {
            if !increasing_flag && !decreasing_flag {
                break;
            }

            if *val > increasing_var {
                decreasing_flag = false;
            } else {
                increasing_flag = false;
            }

            if *val < decreasing_var {
                increasing_flag = false;
            } else {
                decreasing_flag = false;
            }

            if increasing_flag {
                increasing_var = *val;
                increasing_num += 1;
            }

            if decreasing_flag {
                decreasing_var = *val;
                decreasing_num += 1;
            }
        }

        res = std::cmp::max(res, std::cmp::max(increasing_num, decreasing_num));
    }

    res as i32
}

pub fn main() {
    let nums = vec![1, 4, 3, 3, 2];
    println!("{}", longest_monotonic_subarray(nums));
}
