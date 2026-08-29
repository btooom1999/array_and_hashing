fn check(nums: Vec<i32>) -> bool {
    if nums.len() == 1 {
        return true;
    }

    let mut sorted_nums = nums.clone();
    sorted_nums.sort();

    let max = sorted_nums.last().unwrap();
    let mut x = 0;
    let mut flag = false;
    for i in 1..nums.len() {
        if !flag && nums[i - 1] == *max && nums[i] != *max {
            flag = true;
        }

        if flag {
            x += 1;
        }
    }

    for i in 0..nums.len() {
        if nums[i] != sorted_nums[(i + x as usize) % nums.len()] {
            return false;
        }
    }

    true
}

pub fn main() {
    let nums = vec![3, 4, 5, 1, 2];
    println!("{}", check(nums));
}
