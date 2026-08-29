fn is_palindrome(s: String) -> bool {
    let s = s
        .chars()
        .filter_map(|v| {
            if v.is_ascii_alphanumeric() {
                Some(v.to_ascii_lowercase())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let mut l = 0;
    let mut r = s.len();
    while l < r {
        r -= 1;
        if s[l] != s[r] {
            return false;
        }
        l += 1;
    }

    true
}
pub fn main() {

    let s = "A".to_string();
    println!("{}", is_palindrome(s));
}
