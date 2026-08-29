fn check_possibility(nums: Vec<i32>) -> bool {
    let mut max = 0;
    let mut changed = false;

    for i in 1..nums.len() {
        if nums[i] >= nums[max] {
            max = i;
        } else if nums[i] < nums[max] {
            let before = *nums.get(max.wrapping_sub(1)).unwrap_or(&i32::MIN);
            let after = *nums.get(i+1).unwrap_or(&i32::MIN);

            if !changed && before <= nums[i] && nums[i] <= nums[max+1] {
                max = i;
                changed = true;
            } else if !changed && nums[i-1] <= nums[max] && nums[max] <= after {
                changed = true;
            } else {
                return false;
            }
        }
    }

    true
}

pub fn main() {
    let nums = [3,4,2,3].to_vec();
    println!("{}", check_possibility(nums));
}
