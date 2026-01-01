use std::collections::HashMap;

fn majority_element(nums: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut candidate = -1;
    for num in &nums {
        if count == 0 {
            candidate = *num;
        }

        if candidate == *num {
            count += 1;
        } else {
            count -= 1;
        }
    }

    candidate
}

pub fn main() {
    let nums = vec![3, 2, 3];

    println!("{}", majority_element(nums));
}
