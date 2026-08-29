fn is_majority_element(nums: Vec<i32>, target: i32) -> bool {
    let mut l = 0;
    let mut r = nums.len() - 1;

    while l < r {
        let m = (l + r) / 2;
        if nums[m] >= target {
            r = m;
        } else {
            l = m + 1;
        }
    }

    if let (Some(&val1), Some(&val2)) = (nums.get(l), nums.get(l + nums.len() / 2)) {
        val1 == target && val2 == target
    } else {
        false
    }
}

pub fn main() {
    // let nums = vec![2,4,5,5,5,5,5,6,6];
    let nums = vec![10,100,101,101];
    let target = 101;
    println!("{}", is_majority_element(nums, target));
}
