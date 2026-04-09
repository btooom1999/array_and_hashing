fn to_lower_case(mut s: String) -> String {
    unsafe {
        for c in s.as_bytes_mut() {
            if c.is_ascii_uppercase() {
                *c += 32;
            }
        }
    }

    s
}

pub fn main() {
    let s = "Hello".to_string();
    println!("{}", to_lower_case(s));
}
