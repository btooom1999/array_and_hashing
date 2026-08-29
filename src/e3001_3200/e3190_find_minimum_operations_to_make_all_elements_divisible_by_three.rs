fn minimum_operations(nums: Vec<i32>) -> i32 {
    nums.into_iter().fold(0, |acc, num| {
        let remaining = num % 3;
        acc + remaining.min(3-remaining)
    })
}

pub fn main() {
    let nums = [1,2,3,4].to_vec();
    println!("{}", minimum_operations(nums));
}
