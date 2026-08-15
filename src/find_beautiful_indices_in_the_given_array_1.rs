fn calculate_z(s: Vec<u8>, t: Vec<u8>) -> Vec<i32> {
    let m = t.len();
    let s = format!("{}#{}", String::from_utf8(t).unwrap(), String::from_utf8(s).unwrap()).into_bytes();
    let n = s.len();
    let mut res = vec![];
    let mut z = vec![0; n];

    let mut left = 0;
    let mut right = 0;
    for k in 1..n {
        let k1 = k - left;
        if k > right || z[k1] > right - k {
            left = k;
            if k > right { right = k; }
            while right < n && right-left < m && s[right] == s[right-left] {
                right += 1;
            }

            z[k] = right-left;
            right -= 1;
        } else {
            z[k] = z[k1];
        }

        if z[k] == m {
            res.push((k-m-1) as i32);
        }
    }

    res
}

fn beautiful_indices(s: String, a: String, b: String, k: i32) -> Vec<i32> {
    let s= s.into_bytes();
    let a = a.into_bytes();
    let b = b.into_bytes();
    let a = calculate_z(s.clone(), a.clone());
    let mut b = calculate_z(s.clone(), b.clone());
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
    let s = "isawsquirrelnearmysquirrelhouseohmy".to_string();
    let a = "my".to_string();
    let b = "squirrel".to_string();
    let k = 15;
    println!("{:?}", beautiful_indices(s, a, b, k));
}
