use std::collections::HashSet;

fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
    let mut hashset = vec![false; nums.len()];
    let mut duplicate = -1;
    for num in &nums {
        if hashset[(num - 1) as usize] {
            duplicate = *num;
        }

        hashset[(num - 1) as usize] = true;
    }

    if let Some((i, _)) = hashset.into_iter().enumerate().find(|(_, v)| !*v) {
        vec![duplicate, i as i32 + 1]
    } else {
        vec![]
    }
}

pub fn main() {
    let nums = vec![3, 2, 3, 4, 6, 5];
    println!("{:?}", find_error_nums(nums));
}
