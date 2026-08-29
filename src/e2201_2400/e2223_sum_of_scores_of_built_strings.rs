fn sum_scores(s: String) -> i64 {
    let s = s.as_bytes();
    let n = s.len();

    let mut z = vec![0; n];
    let mut left = 0;
    let mut right = 0;
    let mut sum = n as i64;
    for k in 1..n {
        let k1 = k - left;
        if k > right || z[k1] > right - k {
            left = k;
            if k > right { right = k; }
            while right < n && s[right] == s[right-left] {
                right += 1;
            }
            z[k] = right - left;
            right -= 1;
        } else {
            z[k] = z[k1];
        }

        sum += z[k] as i64;
    }

    sum
}

pub fn main() {
    let s = "aaaaaaaa".to_string();
    println!("{}", sum_scores(s));
}
