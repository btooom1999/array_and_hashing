use std::collections::HashMap;

fn arithmetic_triplets(nums: Vec<i32>, diff: i32) -> i32 {
    let mut hashmap = HashMap::new();
    let mut res = 0;
    for num in nums {
        if let Some(count) = hashmap.remove(&(num-diff)) {
            if count >= 2 {
                res += 1;
            }
            hashmap.insert(num, count+1);
        } else {
            hashmap.insert(num,1);
        }
    }

    res
}

pub fn main() {
    let nums = [0,1,4,6,7,10].to_vec();
    let diff = 3;
    println!("{}", arithmetic_triplets(nums, diff));
}
