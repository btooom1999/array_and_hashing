use std::collections::HashSet;

const MOD: i64 = 2i64.pow(45)-1;
const BASE: i64 = 26;

fn distinct_echo_substrings(text: String) -> i32 {
    let text = text.as_bytes();
    let n = text.len();

    let mut hash = vec![0; n+1];
    let mut pow = vec![1; n+1];

    let mul_mod = |a: i64, b: i64, c: i64| -> i64 {
        ((a as i128) * (b as i128) % (c as i128)) as i64
    };

    for i in 0..n {
        hash[i+1] = mul_mod(hash[i], BASE, MOD) + (text[i] - b'a' + 1) as i64;
        pow[i+1] = mul_mod(pow[i], BASE, MOD);
    }

    let get_hash = |l: usize, r: usize| -> i64 {
        (hash[r] + MOD - mul_mod(hash[l], pow[r-l], MOD)) % MOD
    };

    let mut res = HashSet::new();
    for len in 1..=n/2 {
        for i in 0..n-2*len+1 {
            let a = get_hash(i, i+len);
            let b = get_hash(i+len, i+2*len);
            if a == b {
                res.insert(get_hash(i, i+2*len));
            }
        }
    }

    res.len() as i32
}

pub fn main() {
    let text = "abcabcabc".to_string();
    println!("{}", distinct_echo_substrings(text));
}
