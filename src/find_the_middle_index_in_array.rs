fn find_middle_index(nums: Vec<i32>) -> i32 {
    let mut right = nums.iter().sum::<i32>();
    let mut left = 0;
    for (i, pivot) in nums.iter().enumerate() {
        right -= *pivot;
        if left == right {
            return i as i32;
        }
        left += *pivot;
    }

    -1
}

pub fn main() {
    let nums = vec![2, 3, -1, 8, 4];
    println!("{}", find_middle_index(nums));
}
