use std::collections::HashMap;

fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
    let mut sum = 0;
    for num in &nums {
        sum += *num as i64;
    }

    if sum % p as i64 == 0 {
        return 0;
    }

    let need = (sum % p as i64) as i32;
    let mut hashmap = HashMap::<i32, i32>::from([(0, -1)]);
    sum = 0;
    let mut res = i32::MAX;
    for (j, num) in nums.iter().enumerate() {
        sum += *num as i64;
        let prefix = (sum % p as i64) as i32;
        let mut want = (prefix - need) % p;
        if want < 0 {
            want += p;
        }
        if let Some(val) = hashmap.get(&want) {
            res = std::cmp::min(res, j as i32 - *val);
        }

        hashmap.insert(prefix, j as i32);
    }

    if res == nums.len() as i32 { -1 } else { res }
}

pub fn main() {
    let nums = vec![6, 3, 5, 2];
    let p = 9;
    println!("{}", min_subarray(nums, p));
}
