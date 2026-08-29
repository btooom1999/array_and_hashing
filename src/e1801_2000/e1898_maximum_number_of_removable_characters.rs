fn is_subsequence(s: &[u8], p: &[u8]) -> bool {
    let mut l = 0;
    let r = p.len() - 1;
    for &c in s {
        if c == p[l] {
            l += 1;
        }

        if l > r {
            return true;
        }
    }

    false
}

fn maximum_removals(s: String, p: String, removable: Vec<i32>) -> i32 {
    let mut l = 0;
    let mut r = removable.len();

    let s = s.into_bytes();
    let p = p.into_bytes();

    while l <= r {
        let m = (l + r) / 2;
        let mut s = s.clone();
        for i in 0..m {
            s[removable[i] as usize] = b'A';
        }

        let flag = is_subsequence(&s, &p);
        if flag {
            l = m + 1;
        } else {
            r = m - 1;
        }
    }

    r as i32
}

pub fn main() {
    let s = "abcacb".to_string();
    let p = "ab".to_string();
    let removable = [3,1,0].to_vec();
    println!("{}", maximum_removals(s, p, removable));
}
