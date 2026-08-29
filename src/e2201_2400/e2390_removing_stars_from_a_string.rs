fn remove_stars(s: String) -> String {
    let mut res = Vec::new();
    for num in s.as_bytes() {
        if *num != b'*' {
            res.push(*num);
        } else {
            res.pop();
        }
    }

    String::from_utf8(res).unwrap()
}

pub fn main() {
    let s = "leet**cod*e".to_string();
    println!("{}", remove_stars(s));
}
