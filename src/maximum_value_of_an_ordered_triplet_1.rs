fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
    let n = nums.len();
    let mut suffix = vec![i64::MIN; n];
    suffix[n-1] = nums[n-1] as i64;

    for i in (0..n-1).rev() {
        suffix[i] = (nums[i] as i64).max(suffix[i+1]);
    }

    let mut max = i64::MIN;
    let mut min = i64::MAX;
    let mut res = (nums[0] as i64 - nums[1] as i64) * nums[2] as i64;

    for i in 0..nums.len() {
        if min != i64::MAX {
            res = res.max((max - min) * suffix[i]);
        }

        let val = nums[i] as i64;
        if val > max {
            max = val;
            min = i64::MAX;
        } else if val < min {
            min = val;
        }
    }

    res.max(0)
}

pub fn main() {
    // let nums = [8,6,3,13,2,12,19,5,19,6,10,11,9].to_vec();
    let nums = [15,12,2,14,15,18,15,20,14,5,14,14,11,13,7].to_vec();
    println!("{}", maximum_triplet_value(nums));
}
