fn count_hill_valley(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut res = 0;
    let mut i = 0;
    while i < n-1 {
        let mut ni = i+1;
        while ni < n-1 && nums[i] == nums[ni] {
            ni += 1;
        }
        if i>0 {
            res += ((nums[i] > nums[i-1] && nums[i] > nums[ni]) || (nums[i] < nums[i-1] && nums[i] < nums[ni])) as i32;
        }

        i = ni;
    }

    res
}

pub fn main() {
    let nums = [2,4,1,1,6,5].to_vec();
    println!("{}", count_hill_valley(nums));
}
