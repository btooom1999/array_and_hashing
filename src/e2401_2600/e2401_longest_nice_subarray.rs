fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
    let mut num = 0;
    let mut count = 0;
    let mut res = 0;
    let mut i = 0;
    for j in 0..nums.len() {
        while i <= j && num & nums[j] != 0 {
            num ^= nums[i];
            i += 1;
            count -= 1;
        }

        count += 1;
        num |= nums[j];

        res = res.max(count);
    }

    res
}

pub fn main() {
    let nums = [1,3,8,48,10].to_vec();
    println!("{}", longest_nice_subarray(nums));
}
