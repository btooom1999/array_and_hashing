use std::collections::VecDeque;

fn min_max_game(nums: Vec<i32>) -> i32 {
    let mut min = true;
    let mut nums = VecDeque::from(nums);

    while nums.len() > 1 {
        let nums1 = nums.pop_front().unwrap();
        let nums2 = nums.pop_front().unwrap();
        if min {
            nums.push_back(nums1.min(nums2));
        } else {
            nums.push_back(nums1.max(nums2));
        }

        min = !min;
    }

    nums[0]
}

pub fn main() {
    let nums = [70,38,21,22];
    println!("{}", min_max_game(nums.into()));
}
