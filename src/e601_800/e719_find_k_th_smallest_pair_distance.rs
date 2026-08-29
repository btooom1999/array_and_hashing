fn helper(
    nums: &[i32],
    n: usize,
    target: i32,
) -> i32 {
    let mut count = 0;
    for i in 0..n {
        let mut j = i+1;
        while j < n && nums[j]-nums[i] <= target {
            count += 1;
            j += 1;
        }
    }

    count
}

fn smallest_distance_pair(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort_unstable();

    let n = nums.len();
    let mut l = 0;
    let mut r = nums[n-1];

    let mut res = i32::MAX;
    while l < r {
        let m = (l + r) / 2;
        if helper(&nums, n, m) < k {
            l = m + 1;
        } else {
            res = res.min(m);
            r = m;
        }
    }

    res
}

pub fn main() {
    let nums = [9,10,7,10,6,1,5,4,9,8].to_vec();
    let k = 18;
    println!("{}", smallest_distance_pair(nums, k));
}
