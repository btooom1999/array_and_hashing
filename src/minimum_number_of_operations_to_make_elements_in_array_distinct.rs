use std::collections::HashMap;

fn minimum_operations(nums: Vec<i32>) -> i32 {
    let mut hashmap = HashMap::new();
    let mut res = 0;
    let mut errors = 0;
    for &num in &nums {
        let val = hashmap.entry(num).or_insert(0);
        *val += 1;
        if *val == 2 {
            errors += 1;
        }
    }

    if errors == 0 {
        return 0;
    }

    let mut res = 0;
    let mut i = std::cmp::min(2, nums.len());
    for (r, &num) in nums.iter().enumerate() {
        let val = hashmap.get_mut(&num).unwrap();
        *val -= 1;
        if *val == 1 {
            errors -= 1;
        }

        if r == i {
            res += 1;
            i = std::cmp::min(i+3, nums.len());
            if errors == 0 {
                break;
            }
        }
    }

    res
}

pub fn main() {
    let nums = [1,2,3,4,2,3,3,5,7].to_vec();
    println!("{}", minimum_operations(nums));
}
