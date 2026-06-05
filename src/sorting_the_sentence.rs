fn sort_sentence(s: String) -> String {
    let mut s = s.split_whitespace().map(|w| {
        for i in 1..w.len() {
            if let Ok(num) = w[i..].parse::<i32>() {
                return (&w[..i], num);
            }
        }

        unreachable!()
    }).collect::<Vec<_>>();

    s.sort_by_key(|v| v.1);
    s.into_iter().map(|v| v.0.to_string()).collect::<Vec<_>>().join(" ")
}

pub fn main() {
    let s = "is2 sentence4 This1 a3".to_string();
    println!("{}", sort_sentence(s));
}
