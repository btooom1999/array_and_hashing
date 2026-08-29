fn maximum_or(nums: Vec<i32>, k: i32) -> i64 {
    let mut total_bits = [0;32];
    for &num in &nums {
        for i in 0..32 {
            if num & 1 << i != 0 {
                total_bits[i] += 1;
            }
        }
    }

    let mut res = 0;
    for &num in &nums {
        let mut total = 0i64;
        for i in 0..32 {
            if num & 1 << i == 0 || total_bits[i] > 1 {
                total ^= ((total_bits[i] > 0) as i64) << i;
            }
        }

        res = res.max(total | ((num as i64) << k));
    }

    res
}

pub fn main() {
    let nums = [12,9].to_vec();
    let k = 1;
    println!("{}", maximum_or(nums, k));
}
