fn min_moves(nums: Vec<i32>) -> i32 {
    let max = *nums.iter().max().unwrap();
    nums.into_iter().fold(0, |acc, num| acc + max-num)
}

pub fn main() {
    let nums = [2,1,3].to_vec();
    println!("{}", min_moves(nums));
}
