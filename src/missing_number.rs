fn missing_number(mut nums: Vec<i32>) -> i32 {
    for i in 0..nums.len() {
        let idx = if nums[i] == i32::MIN { 0 } else { nums[i].abs() };
        if let Some(val) = nums.get_mut(idx as usize) {
            *val = -(val.abs());
            if *val == 0 {
                *val = i32::MIN;
            }
        }
    }

    for (i, val) in nums.iter().enumerate() {
        if *val >= 0 {
            return i as i32;
        }
    }

    nums.len() as i32
}

pub fn main() {
    let nums = vec![9,6,4,2,3,5,7,0,1];
    println!("{}", missing_number(nums));
}
