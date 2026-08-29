fn minimum_length(s: String) -> i32 {
    let s = s.as_bytes();
    let mut l = 0;
    let mut r = s.len() - 1;
    while l < r {
        if s[l] != s[r] {
            return (r - l + 1) as i32;
        } else {
            let val = s[l];
            while l < r && s[l] == val {
                l += 1;
            }

            let val = s[r];
            while l <= r && s[r] == val {
                r -= 1;
            }
        }
    }

    if l != r {
        0
    } else {
        1
    }
}

pub fn main() {
    let s = "cabaabac".to_string();
    println!("{}", minimum_length(s));
}
