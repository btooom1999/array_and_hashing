fn calculate_z(s: &[u8], t: &[u8]) -> Vec<i32> {
    let n = s.len();
    let m = t.len();
    let mut result = Vec::new();
    let mut z = vec![0; n];

    for i in 0..m {
        if s[i] != t[i] { break; }
        z[0] += 1;
    }

    if z[0] == m {
        result.push(0);
    }
    z[0] = 0;

    let mut left = 0;
    let mut right = 0;
    for k in 1..n {
        let k1 = k - left;
        if k > right || (z[k1] > right - k) {
            left = k;
            if k > right { right = k; }
            while right < n && right-left < m && s[right] == t[right-left] {
                right += 1;
            }

            z[k] = right-left;
            right -= 1;
        } else {
            z[k] = z[k1];
        }

        if z[k] == m {
            result.push(k as i32);
        }
    }

    result
}

fn beautiful_indices(s: String, a: String, b: String, k: i32) -> Vec<i32> {
    let a = calculate_z(s.as_bytes(), a.as_bytes());
    let mut b = calculate_z(s.as_bytes(), b.as_bytes());
    let mut res = vec![];
    for &k1 in a.iter().rev() {
        while b.last().is_some_and(|&k2| (k1 - k2).abs() > k && k1 < k2) {
            b.pop();
        }

        if let Some(&k2) = b.last() {
            if k1 > k2 && (k1 - k2).abs() > k { continue; }
            res.push(k1);
        } else {
            break;
        }


    }

    res.reverse();
    res
}

pub fn main() {
    let s = "ababababazzabababb".to_string();
    let a = "aba".to_string();
    let b = "bb".to_string();
    let k = 10;
    println!("{:?}", beautiful_indices(s, a, b, k));
}
