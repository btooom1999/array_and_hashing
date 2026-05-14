use std::collections::HashMap;

fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
    let mut hashmap = HashMap::new();
    let mut max = (1,1);
    for j in 0..nums.len() {
        if let Some((count, i)) = hashmap.get_mut(&nums[j]) {
            *count += 1;
            if *count > max.0 {
                max = (*count, (j-*i+1) as i32);
            } else if *count == max.0 {
                max.1 = max.1.min((j-*i+1) as i32);
            }
        } else {
           hashmap.insert(nums[j], (1,j));
        }
    }

    max.1
}

pub fn main() {
    let nums = [1,2,2,3,1].to_vec();
    println!("{}", find_shortest_sub_array(nums));
}
