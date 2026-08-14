fn repeated_string_match(a: String, b: String) -> i32 {
    let count = (a.len() + b.len() - 1) / a.len();
    let mut str = a.repeat(count);
    if str.contains(&b) {
        return count as i32;
    }

    str.push_str(&a);
    if str.contains(&b) {
        return (count+1) as i32;
    }

    -1
}

pub fn main() {
    let a = "aaac".to_string();
    let b = "aac".to_string();
    // let a = "abaa".to_string();
    // let b = "aabaa".to_string();
    println!("{}", repeated_string_match(a, b));
}
