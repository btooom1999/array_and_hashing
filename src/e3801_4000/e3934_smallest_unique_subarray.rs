use std::collections::HashMap;

const MOD1: i64 = 2i64.pow(45)-1;
const MOD2: i64 = 2i64.pow(47)-1;
const BASE1: i64 = 23;
const BASE2: i64 = 29;

fn mul_mod(a: i64, b: i64, c: i64) -> i64 {
    (a as i128 * b as i128 % c as i128) as i64
}

fn check(nums: &[i32], k: usize) -> bool {
    let n = nums.len();
    let mut hash1 = 0i64;
    let mut hash2 = 0i64;
    let mut pow1 = 1i64;
    let mut pow2 = 1i64;
    for i in 0..k {
        hash1 = (mul_mod(nums[n-k+i] as i64, pow1, MOD1) + hash1) % MOD1;
        if i+1 < k { pow1 = mul_mod(pow1, BASE1, MOD1); }

        hash2 = (mul_mod(nums[n-k+i] as i64, pow2, MOD2) + hash2) % MOD2;
        if i+1 < k { pow2 = mul_mod(pow2, BASE2, MOD2); }
    }

    let mut count = 0;
    let mut hashmap = HashMap::<_, i32>::new();
    for i in (0..=n-k).rev() {
        if i < n-k {
            hash1 = (hash1 + MOD1 - mul_mod(nums[i+k] as i64, pow1, MOD1)) % MOD1;
            hash1 = mul_mod(hash1, BASE1, MOD1);
            hash1 = (hash1 + nums[i] as i64) % MOD1;

            hash2 = (hash2 + MOD2 - mul_mod(nums[i+k] as i64, pow2, MOD2)) % MOD2;
            hash2 = mul_mod(hash2, BASE2, MOD2);
            hash2 = (hash2 + nums[i] as i64) % MOD2;
        }

        let val = hashmap.entry((hash1, hash2)).or_default();
        *val += 1;
        if *val == 1 { count += 1;}
        else if *val == 2 { count -= 1; }
    }

    count>0
}

fn smallest_unique_subarray(nums: Vec<i32>) -> i32 {
    let mut l = 1;
    let mut r = nums.len();

    while l < r {
        let m = (l+r)/2;
        if check(&nums, m) {
            r = m;
        } else {
            l = m+1;
        }
    }

    l as i32
}

pub fn main() {
    let nums = [1,1,2,2,1].to_vec();
    println!("{}", smallest_unique_subarray(nums));
}
