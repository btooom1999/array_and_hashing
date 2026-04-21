fn maximum_odd_binary_number(s: String) -> String {
    let mut ones = 0;
    let mut zeroes = 0;
    for c in s.chars() {
        if c == '1' {
            ones += 1;
        } else {
            zeroes += 1;
        }
    }

    format!("{}{}1",
        vec!['1'; ones-1].into_iter().collect::<String>(),
        vec!['0'; zeroes].into_iter().collect::<String>()
    )
}

pub fn main() {
    let s = "010".to_string();
    println!("{:?}", maximum_odd_binary_number(s));
}
