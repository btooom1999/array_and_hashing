use std::collections::HashSet;

fn subarray_bitwise_o_rs(arr: Vec<i32>) -> i32 {
    let mut res = HashSet::new();
    let mut prev_nums = HashSet::new();
    for num in arr {
        let mut current = HashSet::new();
        current.insert(num);

        for &or in prev_nums.iter() {
            current.insert(num | or);
        }

        prev_nums = current.clone();
        res.extend(current);
    }

    res.len() as i32
}

fn generate_random_elements() -> Vec<i32> {
    (0..50_001).map(|_| rand::random_range(0..1_000_000_001)).collect()
}

pub fn main() {
    // let arr = generate_random_elements();
    let arr = [1,2,4].to_vec();
    println!("{}", subarray_bitwise_o_rs(arr));
}
