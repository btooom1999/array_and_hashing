fn is_array_special(nums: Vec<i32>) -> bool {
    if nums.len() == 1 {
        return true;
    }

    for i in 1..nums.len() {
        if nums[i] % 2 == 0 && nums[i - 1] % 2 == 0 {
            return false;
        }

        if nums[i] % 2 != 0 && nums[i - 1] % 2 != 0 {
            return false;
        }
    }

    true
}

pub fn main() {
    let nums = vec![4, 3, 1, 6];
    println!("{}", is_array_special(nums));
}
