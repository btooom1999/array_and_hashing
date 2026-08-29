fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut l = 0;
    let mut r = nums.len() as i32;
    while l < r {
        let m = (l + r) / 2;
        if nums[m as usize] == target {
            return m;
        } else if nums[m as usize] < target {
            l = m + 1;
        } else {
            r = m;
        }
    }

    l
}

pub fn main() {
    let nums = vec![1,3];
    let target = 2;
    println!("{}", search_insert(nums, target));
}
