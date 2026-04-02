use std::collections::VecDeque;

fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
    let mut res = vec![VecDeque::new(); nums.len()];
    for (i, mut num) in nums.into_iter().enumerate() {
        while num > 0 {
            res[i].push_front(num % 10);
            num /= 10;
        }
    }

    res.into_iter().flatten().collect()
}

pub fn main() {
    let nums = [13,25,83,77];
    println!("{:?}", separate_digits(nums.into()));
}
