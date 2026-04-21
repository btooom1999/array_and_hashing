fn min_operations(mut nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut zeroes = 0;
    let mut count = 0;
    for i in 0..n-2 {
        if nums[i] == 0 {
            count += 1;
            nums[i] = 1;
            nums[i+1] = (nums[i+1] != 1) as i32;
            nums[i+2] = (nums[i+2] != 1) as i32;
        }
    }

    if nums[n-1] == 0 || nums[n-2] == 0 {
        return -1;
    }

    count
}

pub fn main() {
    let nums = [0,1,1,1,0,0].to_vec();
    println!("{:?}", min_operations(nums));
}
