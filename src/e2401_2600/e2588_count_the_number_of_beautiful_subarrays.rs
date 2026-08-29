use std::collections::HashMap;

fn beautiful_subarrays(nums: Vec<i32>) -> i64 {
    let mut hashmap = HashMap::<_, i64>::new();
    hashmap.insert(0, 1);
    let mut bit = 0;
    let mut res = 0;
    for num in nums {
        bit ^= num;
        let val = hashmap.entry(bit).or_default();
        res += *val;
        *val += 1;
    }

    res
}

pub fn main() {
    // let nums = [4,3,1,2,4].to_vec();
    let nums = [0,0,0].to_vec();
    println!("{}", beautiful_subarrays(nums));
}
