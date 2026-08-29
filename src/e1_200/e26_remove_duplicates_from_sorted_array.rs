fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut i = 1;
    for j in 1..nums.len() {
        if nums[j] != nums[i-1] {
            nums[i] = nums[j];
            i += 1;
        }
    }

    i as i32
}

pub fn main() {
    let mut nums = vec![1, 1, 2];
    println!("{}", remove_duplicates(&mut nums));
}
