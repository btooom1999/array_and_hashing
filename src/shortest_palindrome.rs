fn shortest_palindrome(s: String) -> String {
    let is_palindrome = |a: &[u8], b: &[u8]| -> bool {
        let n = a.len();
        for i in 0..n {
            if a[i] != b[n-i-1] {
                return false;
            }
        }

        true
    };

    let right = s.clone();
    let mut dest = 0;
    let n = s.len();
    let s=  s.as_bytes();

    for i in 1..n {
        if s[i] != s[0] { break; }
        dest = i;
    }

    for j in (1..n/2+1).rev() {
        if j*2 < n {
            let left = &s[..j];
            let right = &s[j+1..j*2+1];
            if is_palindrome(left, right) {
                dest = dest.max(j+left.len());
                break;
            }
        }

        if j*2+1 < n {
            println!();
            let left = &s[..j+1];
            let right = &s[j+1..j*2+2];
            if is_palindrome(left, right) {
                dest = dest.max(j+left.len());
                break;
            }
        }
    }


    let mut left = String::new();
    for i in dest+1..n {
        left = format!("{}{}", s[i] as char, left);
    }

    format!("{}{}", left, right)
}

pub fn main() {
    let s = "aaccaa".to_string();
    println!("{}", shortest_palindrome(s));
}
