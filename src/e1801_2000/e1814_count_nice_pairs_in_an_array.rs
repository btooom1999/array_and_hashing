use std::collections::HashMap;

const MOD: i64 = 1_000_000_007;
fn count_nice_pairs(nums: Vec<i32>) -> i32 {
    let mut hashmap = HashMap::<i32, i32>::new();
    let mut res = 0;
    for num in nums.into_iter() {
        let mut reversed_num = 0;
        let mut x = num;
        while x > 0 {
            reversed_num *= 10;
            reversed_num += x % 10;
            x /= 10;
        }

        let val = hashmap.entry(num - reversed_num).or_default();
        res = ((res as i64 + *val as i64) % MOD) as i32;
        *val += 1;
    }

    res
}

pub fn main() {
    let nums = vec![42,42,11,1,97,10];
    println!("{}", count_nice_pairs(nums));
}
