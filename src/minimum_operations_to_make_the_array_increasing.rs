fn min_operations(nums: Vec<i32>) -> i32 {
    let mut res = 0;
    let mut max = nums[0];
    for &num in nums.iter().skip(1) {
        res += (0).max(max - num + 1);
        max = num.max(max + 1);
    }

    res
}

pub fn main() {
    let nums = [1,5,2,4,1].to_vec();
    println!("{}", min_operations(nums));
}
