fn rearrange_array(mut nums: Vec<i32>) -> Vec<i32> {
    nums.sort();

    let mut i = 2;
    while i < nums.len() {
        (nums[i], nums[i-1]) = (nums[i-1], nums[i]);
        i += 2;
    }

    nums
}

pub fn main() {
    let nums = vec![1,2,3,4,5];
    println!("{:?}", rearrange_array(nums));
}
