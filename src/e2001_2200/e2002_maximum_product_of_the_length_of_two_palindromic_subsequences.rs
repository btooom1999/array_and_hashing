fn is_valid(s: &[u8], bit: usize) -> bool {
    let mut i = 0;
    let mut j = 30;

    while i < j {
        if bit >> i & 1 == 0 {
            i += 1;
        } else if bit >> j & 1 == 0 {
            j -= 1;
        } else if s[i] != s[j] {
            return false;
        } else {
            i += 1;
            j -= 1;
        }
    }

    true
}

fn max_product(s: String) -> i32 {
    let s = s.as_bytes();
    let n = 1 << s.len();
    let mut dp = vec![0; n];
    for bit in 1..n {
        if is_valid(s, bit) {
            dp[bit] = bit.count_ones() as i32;
        }
    }

    let mut res = 0;
    for i in 1..n {
        for j in i+1..n {
            if i & j == 0 {
                res = res.max(dp[i] * dp[j]);
            }
        }
    }

    res
}

pub fn main() {
    let s = "leetcodecom".to_string();
    println!("{}", max_product(s));
}
