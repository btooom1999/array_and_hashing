fn is_subsequence(s: String, t: String) -> bool {
    if s.is_empty() { return true; }

    let arr = s.chars().collect::<Vec<_>>();
    let mut i = 0;

    for char in t.chars() {
        if let Some(val) = arr.get(i) {
            if *val == char {
                i += 1;
            }
        } else {
            break;
        }
    }

    i == arr.len()
}

pub fn main() {
    let s = "axc".to_string();
    let t = "ahbgdc".to_string();
    println!("{}", is_subsequence(s, t));
}
