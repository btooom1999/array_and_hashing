fn find_duplicate(mut nums: Vec<i32>) -> i32 {
    for i in 0..nums.len() {
        let k = nums[i].unsigned_abs() as usize - 1;
        if nums[k] > 0 {
            nums[k] *= -1;
        } else {
            return nums[i].abs();
        }
    }

    -1
}

pub fn main() {
    let nums = [1,3,4,2,2].to_vec();
    println!("{}", find_duplicate(nums));
}
