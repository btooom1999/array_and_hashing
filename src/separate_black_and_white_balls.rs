fn minimum_steps(mut s: String) -> i64 {
    let mut ones = 0;
    let mut swap = 0;
    for c in s.chars() {
        match c {
            '0' => swap += ones,
            '1' => ones += 1,
            _ => unreachable!(),
        }
    }

    swap
}

pub fn main() {
    let s = "101".to_string();
    println!("{}", minimum_steps(s));
}
