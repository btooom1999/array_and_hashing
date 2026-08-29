fn xor_beauty(nums: Vec<i32>) -> i32 {
    nums.into_iter().fold(0, |acc, num| acc ^ num)
}

pub fn main() {
    let nums = [1,4].to_vec();
    println!("{}", xor_beauty(nums));
}
