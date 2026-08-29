fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut l = 0;
    let mut r = nums.len() as i32 - 1;
    while r > 0 && l < r {
        let m = (r + l) / 2;
        if nums[m as usize] == target {
            return m;
        } else if nums[m as usize] < target {
            l = m + 1;
        } else {
            r = m - 1;
        }
    }

    if nums[l as usize] == target { l } else { -1 }
}

pub fn main() {
    let nums = vec![5, 6];
    let target = -5;
    println!("{}", search(nums, target));
}
