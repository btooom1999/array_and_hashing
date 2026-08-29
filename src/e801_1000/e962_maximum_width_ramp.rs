fn max_width_ramp(nums: Vec<i32>) -> i32 {
    let mut max_right = vec![0; nums.len()];
    for i in (0..nums.len()).rev() {
        max_right[i] = std::cmp::max(nums[i], *max_right.get(i+1).unwrap_or(&0));
    }

    let mut l = 0;
    let mut res = 0;
    for r in 0..nums.len() {
        while nums[l] > max_right[r] {
            l += 1;
        }

        res = std::cmp::max(res, r - l);
    }

    res as i32
}

pub fn main() {
    let nums = vec![6,0,8,2,1,5];
    println!("{}", max_width_ramp(nums));
}
