fn sum_of_beauties(nums: Vec<i32>) -> i32 {
    let mut res = 0;
    let n = nums.len();
    let mut suffix = vec![nums[n-1]; n];
    for i in (1..n-1).rev() {
        suffix[i] = nums[i].min(suffix[i+1]);
    }

    let mut min_left = nums[0];
    for i in 1..n-1 {
        if min_left < nums[i] && nums[i] < suffix[i+1] {
            res += 2;
        } else if nums[i-1] < nums[i] && nums[i] < nums[i+1] {
            res += 1;
        }

        min_left = min_left.max(nums[i]);
    }

    res
}

pub fn main() {
    let nums = vec![10,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19].to_vec();
    println!("{}", sum_of_beauties(nums));
}
