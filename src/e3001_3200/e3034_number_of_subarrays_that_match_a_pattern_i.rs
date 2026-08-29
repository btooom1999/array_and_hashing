const MOD: i64 = 2i64.pow(61)-1;
const BASE: i64 = 31;

#[inline]
fn mul_mod(a: i64, b: i64, c: i64) -> i64 {
    (a as i128 * b as i128 % c as i128) as i64
}

fn count_matching_subarrays(nums: Vec<i32>, pattern: Vec<i32>) -> i32 {
    let mut need = 0;
    let k = pattern.len() + 1;
    for num in pattern {
        need = (mul_mod(need, BASE, MOD) + (num as i64 + 2)) % MOD;
    }

    let mut pow = 1;
    for _ in 2..k {
        pow = mul_mod(pow, BASE, MOD);
    }

    let mut hash = 0;
    let mut res = 0;
    let mut i = 1;
    for (j, &num) in nums.iter().enumerate().skip(1) {
        if j >= k {
            let cmp = nums[i].cmp(&nums[i-1]) as i64 + 2;
            hash = (hash + MOD - mul_mod(cmp, pow, MOD)) % MOD;
            i += 1;
        }

        let cmp = num.cmp(&nums[j-1]) as i64 + 2;
        hash = mul_mod(hash, BASE, MOD) + cmp;
        if hash == need {
            res += 1;
        }
    }

    res
}

pub fn main() {
    let nums = [1,4,4,1,3,5,5,3].to_vec();
    let pattern = [1,0,-1].to_vec();
    println!("{}", count_matching_subarrays(nums, pattern));
}
