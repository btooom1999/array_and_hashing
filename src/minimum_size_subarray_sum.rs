use std::collections::HashMap;

fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    let mut sum = 0;
    let mut res = i32::MAX;
    let mut hashmap = HashMap::<i32, i32>::from([(0, -1)]);

    for (j, num) in nums.iter().enumerate() {
        sum += *num;
        let mut x = 0;
        loop {
            if sum - target - x < 0 {
                break;
            }

            if let Some(i) = hashmap.get(&(sum - target - x)) {
                res = std::cmp::min(res, j as i32 - *i);
                break;
            } else {
                x += 1;
            }
        }

        hashmap.entry(sum).or_insert(j as i32);
    }

    if res == i32::MAX {
        return 0;
    }

    res
}

pub fn main() {
    let target = 11;
    let nums = vec![1, 2, 3, 4, 5];
    println!("{}", min_sub_array_len(target, nums));
}
