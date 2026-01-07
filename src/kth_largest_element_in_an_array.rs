const N: i32 = 10_000;
fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    let mut hashmap = vec![0; (N * 2 + 1) as usize];
    for num in &nums {
        hashmap[(*num + N) as usize] += 1;
    }

    let mut c = k;
    for (x , count) in hashmap.iter().enumerate().rev() {
        if *count == 0 {
            continue;
        }

        c -= count;
        if c <= 0 {
            return x as i32 - N;
        }
    }

    -1
}

pub fn main() {
    let nums = vec![-1000, 0, 1, -1];
    let k = 2;
    println!("{}", find_kth_largest(nums, k));
}
