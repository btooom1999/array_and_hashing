use std::collections::{HashMap, HashSet};

const MOD1: i64 = 2i64.pow(45)-1;
const BASE1: i64 = 23;
const MOD2: i64 = 2i64.pow(47)-1;
const BASE2: i64 = 29;

#[inline]
fn mul_mod(a: i64, b: i64, c: i64) -> i64 {
    (a as i128 * b as i128 % c as i128) as i64
}

fn check(
    paths: &[Vec<i32>],
    k: usize,
) -> bool {
    let mut hashmap = HashMap::<_, HashSet<_>>::new();
    for (key, path) in paths.iter().enumerate() {
        let n = path.len();
        if n < k { break; }

        let mut hashed1 = 0i64;
        let mut hashed2 = 0i64;
        let mut pow1 = 1i64;
        let mut pow2 = 1i64;
        for i in 0..k {
            hashed1 = (mul_mod(path[n-k+i] as i64 + 1, pow1, MOD1) + hashed1) % MOD1;
            if i+1 < k { pow1 = mul_mod(pow1, BASE1, MOD1); }

            hashed2 = (mul_mod(path[n-k+i] as i64 + 1, pow2, MOD2) + hashed2) % MOD2;
            if i+1 < k { pow2 = mul_mod(pow2, BASE2, MOD2); }
        }

        for i in (0..=n-k).rev() {
            if i < n-k {
                hashed1 = (hashed1 + MOD1 - mul_mod(path[i+k] as i64 + 1, pow1, MOD1)) % MOD1;
                hashed1 = mul_mod(hashed1, BASE1, MOD1);
                hashed1 = (hashed1 + path[i] as i64 + 1) % MOD1;

                hashed2 = (hashed2 + MOD2 - mul_mod(path[i+k] as i64 + 1, pow2, MOD2)) % MOD2;
                hashed2 = mul_mod(hashed2, BASE2, MOD2);
                hashed2 = (hashed2 + path[i] as i64 + 1) % MOD2;
            }

            let val = hashmap.entry((hashed1, hashed2)).or_default();
            val.insert(key);
            if val.len() == paths.len() { return true; }
        }
    }

    false
}

fn longest_common_subpath(_n: i32, paths: Vec<Vec<i32>>) -> i32 {
    let n = paths.iter().min_by_key(|v| v.len()).unwrap().len() as i32;
    let mut l = 0;
    let mut r = n+1;
    while l < r {
        let m = (l+r)/2;
        let val = check(&paths, m as usize);
        if val {
            l = m+1;
        } else {
            r = m;
        }
    }

    (l-1).max(0)
}

pub fn main() {
    let n = 5;
    let paths = vec![vec![0,1,2,3,4], vec![2,3,4], vec![4,0,1,2,3]];
    println!("{}", longest_common_subpath(n, paths));
}
