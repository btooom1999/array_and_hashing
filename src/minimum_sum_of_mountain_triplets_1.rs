fn minimum_sum(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut prefix = vec![i32::MAX; n];
    prefix[0] = nums[0];
    let mut suffix = vec![i32::MAX; n];
    suffix[n-1] = nums[n-1];

    let mut res = i32::MAX;
    for i in 1..n-1 {
        prefix[i] = nums[i].min(prefix[i-1]);
        suffix[n-i-1] = nums[n-i-1].min(suffix[n-i]);
    }

    for i in 1..n-1 {
        if nums[i] > prefix[i-1] && nums[i] > suffix[i+1] {
            res = res.min(nums[i]+prefix[i-1]+suffix[i+1]);
        }
    }

    if res == i32::MAX { -1 } else { res }
}

pub fn main() {
    let nums = [1,3,4,2,5].to_vec();
    println!("{}", minimum_sum(nums));
}
