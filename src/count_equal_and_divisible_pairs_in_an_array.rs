use std::collections::HashMap;

fn count_pairs(nums: Vec<i32>, k: i32) -> i32 {
    let mut hashmap = HashMap::<i32, Vec<usize>>::with_capacity(nums.len());
    let mut res = 0;
    for (j, num) in nums.into_iter().enumerate() {
        let vec = hashmap.entry(num).or_default();
        if !vec.is_empty() {
            for i in vec.iter() {
                if i * j % k as usize == 0 {
                    res += 1;
                }
            }
        }

        vec.push(j);
    }

    res
}

pub fn main() {
    let nums = vec![3,1,2,2,2,1,3];
    let k = 2;
    println!("{}", count_pairs(nums, k));
}
