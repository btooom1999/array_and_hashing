fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
    let mut count = 0;
    for i in 0..32 {
        let mut ones = 0;
        for &num in &nums {
            if num >> i & 1 == 1 {
                ones += 1;
            }
        }
        if (k >> i & 1 == 1 && ones % 2 == 0)
        || (k >> i & 1 == 0 && ones % 2 == 1) {
            count += 1;
        }
    }

    count
}

pub fn main() {
    let nums = [2,1,3,4].to_vec();
    let k = 1;
    println!("{}", min_operations(nums, k));
}
