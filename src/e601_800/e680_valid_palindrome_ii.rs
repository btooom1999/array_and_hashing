fn valid_palindrome(s: String) -> bool {
    let s = s.chars().collect::<Vec<_>>();
    let mut l = 0;
    let mut r = s.len();
    let mut valid_1 = 1;
    while l < r {
        r -= 1;
        if valid_1 < 0 {
            break;
        }
        if s[l] != s[r] {
            valid_1 -= 1;
            continue;
        }

        l += 1;
    }

    l = 0;
    r = s.len();
    let mut valid_2 = 1;
    while l < r {
        r -= 1;
        if valid_2 < 0 {
            break;
        }
        if s[l] != s[r] {
            r += 1;
            valid_2 -= 1;
        }

        l += 1;
    }

    valid_1 >= 0 || valid_2 >= 0
}

pub fn main() {
    let s = "deeee".to_string();
    println!("{}", valid_palindrome(s));
}

