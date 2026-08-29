fn has_trailing_zeros(nums: Vec<i32>) -> bool {
    let mut count = 0;
    for num in nums {
        if 1 & num == 0 {
            count += 1;
        }
        if count == 2 {
            return true;
        }
    }

    false
}

pub fn main() {
    let nums = [1,2,3,4,5].to_vec();
    println!("{}", has_trailing_zeros(nums));
}
