fn calculate_z(s: Vec<u8>, pattern: Vec<u8>) -> Vec<usize> {
    let m = pattern.len();
    let s = format!("{}#{}", String::from_utf8(pattern).unwrap(), String::from_utf8(s).unwrap()).into_bytes();
    let mut left = 0;
    let mut right = 0;
    let mut z = vec![0; s.len()];
    let n = s.len();
    for k in 1..n {
        let k1 = k - left;
        if k > right || z[k1] > right - k {
            left = k;
            right = right.max(k);
            while right < n && s[right] == s[right-left] {
                right += 1;
            }
            z[k] = right - left;
            right -= 1;
        } else {
            z[k] = z[k1];
        }
    }

    z[m+1..].to_vec()
}

fn min_starting_index(s: String, pattern: String) -> i32 {
    let n = s.len();
    let m = pattern.len();
    let mut s = s.into_bytes();
    let mut pattern = pattern.into_bytes();
    let z = calculate_z(s.clone(), pattern.clone());

    s.reverse();
    pattern.reverse();
    let mut reverse_z = calculate_z(s.clone(), pattern.clone());
    reverse_z.reverse();

    for i in 0..=n-m {
        let j = i + z[i] + (m - z[i]) - 1;
        if z[i] + reverse_z[j] + 1 >= m {
            return i as i32;
        }
    }

    -1
}

pub fn main() {
    // let s = "abcdefg".to_string();
    // let pattern = "bcdffg".to_string();
    let s = "abcd".to_string();
    let pattern = "dba".to_string();
    println!("{:?}", min_starting_index(s, pattern));
}
