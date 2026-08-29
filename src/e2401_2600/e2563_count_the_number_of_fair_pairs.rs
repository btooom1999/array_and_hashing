fn count_seq(nums: &[i32], target: i32) -> i64 {
    let mut res = 0;
    let mut l = 0;
    let mut r = nums.len() - 1;

    while l < r {
        if nums[l] + nums[r] <= target {
            res += r as i64 - l as i64;
            l += 1;
        } else {
            r -= 1;
        }
    }

    res
}

fn count_fair_pairs(mut nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
    nums.sort();

    count_seq(&nums, upper) - count_seq(&nums, lower - 1)
}

pub fn main() {
    let nums = [0,1,7,4,4,5].to_vec();
    let lower = 3;
    let upper = 6;
    println!("{}", count_fair_pairs(nums, lower, upper));
}
