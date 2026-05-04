use std::collections::HashSet;

fn sum_of_largest_primes(s: String) -> i64 {
    let mut hashset = HashSet::new();
    let s = s.as_bytes();
    for i in 0..s.len() {
        let mut res = 0;
        for j in i..s.len() {
            res = res * 10 + (s[j]-b'0') as i64;
            if res != 1 && !(2..=res.isqrt()).any(|num| res % num == 0) {
                hashset.insert(res);
            }
        }
    }

    let mut hashset = hashset.into_iter().collect::<Vec<_>>();
    hashset.sort_unstable();

    let n = hashset.len();

    *hashset.get(n.wrapping_sub(1)).unwrap_or(&0)
    + *hashset.get(n.wrapping_sub(2)).unwrap_or(&0)
    + *hashset.get(n.wrapping_sub(3)).unwrap_or(&0)
}

pub fn main() {
    let s = "589".to_string();
    println!("{:?}", sum_of_largest_primes(s));
}
