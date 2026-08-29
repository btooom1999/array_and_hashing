fn to_hex(num: i32) -> String {
    let mut res = Vec::new();
    let mut num = num as u32;
    while num > 0 {
        let val = num % 16;
        if val < 10 {
            res.push((val+48) as u8);
        } else {
            res.push((val+87) as u8);
        }
        num /= 16;
    }

    res.reverse();
    if res.is_empty() { "0".to_string() } else { String::from_utf8(res).unwrap() }
}

pub fn main() {
    let num = -2;
    println!("{}", to_hex(num));
}
