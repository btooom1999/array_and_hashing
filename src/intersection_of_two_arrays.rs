use std::collections::HashMap;

fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut hashmap = HashMap::<i32, i32>::new();
    for num in &nums1 {
        hashmap.insert(*num, 1);
    }

    let mut res = Vec::new();
    for num in &nums2 {
        if let Some(val) = hashmap.get_mut(num) {
            *val += 1;
            if *val == 2 {
                res.push(*num);
            }
        }
    }

    res
}

pub fn main() {
    let nums1 = vec![1, 2, 2, 1];
    let nums2 = vec![2, 2];
    println!("{:?}", intersection(nums1, nums2));
}
