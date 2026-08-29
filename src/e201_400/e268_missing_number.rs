pub fn missing_number(nums: Vec<i32>) -> i32 {
    let mut n = nums.len() as i32;
    for (i, num) in nums.into_iter().enumerate() {
        n ^= i as i32 ^ num;
    }

    n
}

pub fn main() {
    let nums = vec![3,0,1];
    println!("{}", missing_number(nums));
}
