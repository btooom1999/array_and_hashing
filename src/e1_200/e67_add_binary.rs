fn add_binary(a: String, b: String) -> String {
    let mut excess = 0;
    let mut a = a.into_bytes();
    let mut b = b.into_bytes();
    let mut vec = Vec::new();
    while a.last().is_some() || b.last().is_some() || excess == 1 {
        let a = (a.pop().unwrap_or(b'0') - b'0') as i32;
        let b = (b.pop().unwrap_or(b'0') - b'0') as i32;

        vec.push((a^b^excess) as u8 + b'0');
        excess = (a+b+excess)/2;
    }

    vec.reverse();

    String::from_utf8(vec).unwrap()
}

pub fn main() {
    let a = "11".to_string();
    let b = "11".to_string();
    println!("{}", add_binary(a, b));
}
