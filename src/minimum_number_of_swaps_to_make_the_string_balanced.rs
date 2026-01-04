fn min_swaps(s: String) -> i32 {
    let mut close = 0;
    let mut max_close = 0;
    for c in s.chars() {
        if c == ']' {
            close += 1;
        } else {
            close -= 1;
        }
        max_close = max_close.max(close);
    }

    (max_close + 1) / 2
}

pub fn main() {
    let s = "][][".to_string();
    println!("{}", min_swaps(s));
}
