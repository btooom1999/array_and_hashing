fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut l = 0;
    let mut r = nums.len() - 1;

    while l <= r {
        if nums[l] >= target {
            return l as i32;
        }
        if nums[r] <= target {
            return r as i32 + (nums[r] != target) as i32;
        }

        l += 1;
        r -= 1;
    }

    l as i32
}

pub fn main() {
    let nums = vec![1,3];
    let target = 2;
    println!("{}", search_insert(nums, target));
}
