fn query_string(s: String, n: i32) -> bool {
    (1..=n).all(|i| s.contains(&format!("{i:b}")))
}

pub fn main() {
    let s = "0110".to_string();
    let n = 4;
    println!("{}", query_string(s, n));
}
