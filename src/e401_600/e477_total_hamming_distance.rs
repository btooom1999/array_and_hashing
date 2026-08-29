fn total_hamming_distance(nums: Vec<i32>) -> i32 {
    let mut res = 0;
    let n = nums.len() as i32;
    for i in (0..32).rev() {
        let mut count_zero = 0;
        for &num in &nums {
            if 1 << i & num == 0 {
                count_zero += 1;
            }
        }

        res += count_zero * (n-count_zero);
    }

    res
}

pub fn main() {
    let nums = [4,14,2].to_vec();
    println!("{}", total_hamming_distance(nums));
}
