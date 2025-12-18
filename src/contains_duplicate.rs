use std::collections::HashSet;

fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut hashmap: HashSet<i32> = HashSet::new();
    let mut duplicate = false;

    for i in nums {
        if !hashmap.insert(i) {
            return true;
        }
    }

    false
}

pub fn main() {
    let nums = vec![1, 2, 3, 1];

    println!("{}", contains_duplicate(nums));
}
