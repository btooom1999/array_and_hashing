fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
    let mut res = 1;

    for i in 0..nums.len() {
        if res > nums.len() - i {
            break;
        }

        let mut increasing_arr = Vec::from([nums[i]]);
        let mut decreasing_arr = Vec::from([nums[i]]);
        let mut increase = true;
        let mut decrease = true;

        for val in &nums[i + 1..] {
            if !increase && !decrease {
                break;
            }

            if *val > increasing_arr[increasing_arr.len() - 1] {
                decrease = false;
            } else {
                increase = false;
            }

            if *val < decreasing_arr[decreasing_arr.len() - 1] {
                increase = false;
            } else {
                decrease = false;
            }

            if increase {
                increasing_arr.push(*val);
            }

            if decrease {
                decreasing_arr.push(*val);
            }
        }

        res = std::cmp::max(
            res,
            std::cmp::max(increasing_arr.len(), decreasing_arr.len()),
        )
    }

    res as i32
}

pub fn main() {
    let nums = vec![1, 4, 3, 3, 2];
    println!("{}", longest_monotonic_subarray(nums));
}
