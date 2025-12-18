use std::collections::HashSet;

pub fn main() {
    let mut hashmap: HashSet<i32> = HashSet::new();
    let nums = [1, 2, 3, 1];
    let mut duplicate = false;

    for i in nums {
        if !hashmap.insert(i) {
            duplicate = true;
            break;
        }
    }

    println!("duplicate: {}", duplicate);
}
