fn is_monotonic(nums: Vec<i32>) -> bool {
    if nums.len() == 1 {
        return true;
    }

    let mut increase = true;
    let mut decrease = true;
    for i in 1..nums.len() {
        if !increase && !decrease {
            return false;
        }

        if nums[i] > nums[i - 1] {
            decrease = false;
        }

        if nums[i] < nums[i - 1] {
            increase = false;
        }
    }

    increase || decrease
}

pub fn main() {
    let nums = vec![1, 2, 2, 3];
    println!("{}", is_monotonic(nums));
}
