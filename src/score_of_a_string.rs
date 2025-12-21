fn score_of_string(s: String) -> i32 {
    let s = s.as_bytes();
    let mut res = 0;
    for i in 0..s.len()-1 {
        res += (s[i] as i32 - s[i+1] as i32).abs();
    }

    res
}

pub fn main() {
    let s = "hello".to_string();

    let res = score_of_string(s);
    println!("{}", res);
}
