fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
    let mut zero = None as Option<usize>;
    let mut i = 0;
    while i < nums.len() {
        if nums[i] != 0 {
            if i + 1 < nums.len() && nums[i] == nums[i+1] {
                nums[i] += nums[i];
                nums[i+1] = 0;
            }
            if let Some(pos) = zero {
                (nums[i], nums[pos]) = (nums[pos], nums[i]);
                zero = Some(pos + 1);
            }
        } else if zero.is_none() {
            zero = Some(i);
        }

        i += 1;
    }

    nums
}

pub fn main() {
    let nums = vec![1,2,2,1,1];
    println!("{:?}", apply_operations(nums));
}
