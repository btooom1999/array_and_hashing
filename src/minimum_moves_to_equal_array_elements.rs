fn min_moves(nums: Vec<i32>) -> i32 {
    let mut total = 0;
    let mut min = i32::MAX;
    for &num in &nums {
        min = min.min(num);
        total += num;
    }

    total - min * nums.len() as i32
}

pub fn main() {
    let nums = [1,2,3].to_vec();
    println!("{}", min_moves(nums));
}
