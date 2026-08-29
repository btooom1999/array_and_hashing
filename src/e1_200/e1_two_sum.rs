use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hashmap: HashMap<i32, usize> = HashMap::new();

    for (i, num) in nums.iter().enumerate() {
        if let Some(j) = hashmap.get(&(target - num)) {
            return vec![j.to_owned() as i32, i as i32];
        } 
            
        hashmap.insert(num.to_owned(), i);
    }

    vec![]
}

pub fn main() {
    let nums = [2, 7, 11, 15].to_vec();
    let target = 9;

    println!("{:?}", two_sum(nums, target));
}
