fn make_good(s: String) -> String {
    let s = s.as_bytes().to_vec();
    let mut res = Vec::new();
    for num in &s {
        if res.last().is_some_and(|&v| v == num + 32 || v == num - 32) {
            res.pop();
        } else {
            res.push(*num);
        }
    }

    String::from_utf8(res).unwrap()
}

pub fn main() {
    let s = "leEeetcode".to_string();
    println!("{}", make_good(s));
}
