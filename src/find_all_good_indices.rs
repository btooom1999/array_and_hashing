fn good_indices(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let n = nums.len();
    let mut prefix = vec![1; n+1];
    let mut suffix = vec![1; n+1];

    for i in 1..n {
        if nums[i] <= nums[i-1] {
            prefix[i] = prefix[i-1] + 1;
        }
    }

    for i in (0..n-1).rev() {
        if nums[i] <= nums[i+1] {
            suffix[i] = suffix[i+1] + 1;
        }
    }

    let mut res = vec![];
    for i in k..n-k {
        if prefix[i-1].min(suffix[i+1]) >= k {
            res.push(i as i32);
        }
    }

    res
}

pub fn main() {
    let nums = [2,1,1,2].to_vec();
    let k = 2;
    println!("{:?}", good_indices(nums, k));
}
