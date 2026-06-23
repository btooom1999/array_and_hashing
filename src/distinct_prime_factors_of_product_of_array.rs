use std::collections::HashSet;

fn distinct_prime_factors(nums: Vec<i32>) -> i32 {
    let mut hashset = HashSet::new();
    for mut num in nums {
        if num == 1 {
            hashset.insert(num);
        } else {
            while num > 1 {
                if let Some(i) = (2..=num.isqrt()).find(|&i| num % i == 0) {
                    hashset.insert(i);
                    num /= i;
                } else {
                    hashset.insert(num);
                    break;
                }
            }
        }

    }

    hashset.len() as i32
}

pub fn main() {
    let nums = [2,4,3,7,10,6].to_vec();
    println!("{}", distinct_prime_factors(nums));
}
