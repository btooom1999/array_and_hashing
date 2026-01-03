use std::collections::HashMap;

const MOD: i64 = 1_000_000_007;

fn num_of_subarrays(arr: Vec<i32>) -> i32 {
    let all = ((arr.len() as i64 * (arr.len() as i64 + 1)) / 2);
    let mut hashmap = HashMap::<i32, i64>::from([(0, 1)]);
    let mut sum = 0;
    let mut res = 0;
    for num in &arr {
        sum += *num as i64;
        if let Some(val) = hashmap.get(&((sum % 2) as i32)) {
            res += *val;
        }

        *hashmap.entry((sum % 2) as i32).or_default() += 1;
    }

    ((all - res) % MOD) as i32
}

pub fn main() {
    let arr = vec![1, 3, 5];
    println!("{}", num_of_subarrays(arr));
}
