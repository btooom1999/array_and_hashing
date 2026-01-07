use std::collections::BTreeSet;

fn third_max(nums: Vec<i32>) -> i32 {
    let mut btree_set = BTreeSet::<i32>::new();
    for num in &nums {
        btree_set.insert(*num);
    }

    if btree_set.len() < 3 {
        return *btree_set.last().unwrap();
    }

    *btree_set.iter().nth(btree_set.len() - 3).unwrap()
}

pub fn main() {
    let nums = vec![3,2,1];
    println!("{}", third_max(nums));
}
