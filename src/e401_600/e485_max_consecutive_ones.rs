fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut temp_max = 0;

    for num in nums.into_iter() {
        if num != 1 {
            temp_max = 0;
        } else {
            temp_max += 1;
            max = std::cmp::max(temp_max, max);
        }
    }

    max
}

pub fn main() {
    let nums = vec![1,1,0,1,1,1];
    println!("{}", find_max_consecutive_ones(nums));
}
