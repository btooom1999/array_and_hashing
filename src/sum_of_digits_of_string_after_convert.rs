fn get_lucky(s: String, mut k: i32) -> i32 {
    let mut num = String::new();
    for &b in s.as_bytes() {
        let b = (b - b'a') as i32 + 1;
        num.push_str(&b.to_string());
    }


    while k > 0 {
        let mut temp = 0;
        while let Some(num) = num.pop() {
            temp += (num as u8 - b'0') as i32;
        }

        k -= 1;
        num = temp.to_string();

    }

    num.parse().unwrap()
}

pub fn main() {
    let s = "hwmqsaqvrliksiobdtbtxiztnextxsvpoqeyfvxlnrcwlaqh".to_string();
    let k = 9;
    println!("{}", get_lucky(s, k));
}
