use std::collections::HashSet;

fn min_impossible_or(nums: Vec<i32>) -> i32 {
    let hashset = nums.into_iter().collect::<HashSet<_>>();

    for i in 0..32 {
        if !hashset.contains(&(1 << i)) {
            return 1 << i;
        }
    }

    unreachable!()
}

pub fn main() {
    let nums = [2,1].to_vec();
    println!("{}", min_impossible_or(nums));
}
