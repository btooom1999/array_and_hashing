fn check_array(mut nums: Vec<i32>, k: i32) -> bool {
    let k = k as usize;
    let n = nums.len();
    let mut prefix = vec![0; n];
    let mut prefix_sum = 0;

    for i in 0..n {
        prefix_sum += prefix[i];
        nums[i] += prefix_sum;

        if nums[i] < 0 { return false; }
        if nums[i] == 0 { continue; }
        if i+k >= n { return false; }

        prefix_sum -= nums[i];
        prefix[i+k] += nums[i];
    }

    true
}

pub fn main() {
    let nums = [2,2,3,1,1,0].to_vec();
    let k = 3;
    println!("{}", check_array(nums, k));
}
