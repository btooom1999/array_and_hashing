fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    let mut l = 0;
    let mut sum = 0;
    let mut res = i32::MAX;
    for (r, num) in nums.iter().enumerate() {
        sum += *num;
        while sum >= target && l <= r {
            res = std::cmp::min(res, (r - l + 1) as i32);
            sum -= nums[l];
            l += 1;
        }
    }

    if res == i32::MAX {
        return 0;
    }

    res
}

pub fn main() {
    let target = 7;
    let nums = vec![2, 3, 1, 2, 4, 3];
    println!("{}", min_sub_array_len(target, nums));
}
