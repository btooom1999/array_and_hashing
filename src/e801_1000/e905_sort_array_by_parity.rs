fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
    let mut i = 0 as i32;
    let mut j = (nums.len() - 1) as i32;
    while i < j {
        if nums[i as usize] % 2 == 0 {
            i += 1;
        }
        else if nums[j as usize] % 2 != 0 {
            j -= 1;
        } else {
            (nums[i as usize], nums[j as usize]) = (nums[j as usize], nums[i as usize]);
            i += 1;
            j -= 1;
        }
    }

    nums
}

pub fn main() {
    let nums = vec![0, 1];
    println!("{:?}", sort_array_by_parity(nums));
}
