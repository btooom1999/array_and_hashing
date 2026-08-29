use std::collections::{HashMap, HashSet};

fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
    let target = nums.clone().into_iter().collect::<HashSet<_>>().len();
    let mut hashmap = HashMap::<_, i32>::new();

    let mut res = 0;
    let n = nums.len();
    let mut i = 0;
    for j in 0..n {
        *hashmap.entry(nums[j]).or_default() += 1;

        while hashmap.len() == target {
            res += (n-j) as i32;
            *hashmap.get_mut(&nums[i]).unwrap() -= 1;
            if hashmap[&nums[i]] == 0 {
                hashmap.remove(&nums[i]);
            }
            i += 1;
        }
    }

    res
}

pub fn main() {
    let nums = [1,3,1,2,2].to_vec();
    println!("{}", count_complete_subarrays(nums));
}
