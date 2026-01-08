fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
    let n = nums.len();
    for num in nums.iter_mut() {
        if *num < 1 || *num > (n as i32) {
            *num = i32::MAX;
        }
    }

    for i in 0..n {
        let val = nums[i].abs();
        if val >= 1 && val <= n as i32 {
            nums[(val - 1) as usize] = -nums[(val - 1) as usize].abs();
        }
    }

    let mut need = 1;
    for (i, num) in nums.iter().enumerate() {
        if *num > 0 {
            return i as i32 + 1;
        }

        need += 1;
    }


    need
}

pub fn main() {
    let nums = vec![1,2,2];
    println!("{}", first_missing_positive(nums));
}
