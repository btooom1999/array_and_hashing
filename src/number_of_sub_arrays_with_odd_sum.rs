use std::collections::HashMap;

const MOD: i64 = 1_000_000_007;

fn num_of_subarrays(arr: Vec<i32>) -> i32 {
    let mut prefix = 0;
    let mut odd_count = 0;
    let mut even_count = 0;
    let mut res = 0;
    for num in &arr {
        prefix += *num as i64;
        if prefix % 2 == 1 {
            odd_count += 1;
            res += even_count + 1;
        } else {
            even_count += 1;
            res += odd_count;
        }

        res %= MOD;
    }

    res as i32
}

pub fn main() {
    let arr = vec![1, 3, 5];
    println!("{}", num_of_subarrays(arr));
}
