fn digit_sum(mut s: String, k: i32) -> String {
    while s.len() as i32 > k {
        let mut j = k as usize-1;
        let mut res = String::new();
        let mut temp = 0;
        for (i, c) in s.chars().enumerate() {
            temp += (c as u8 - b'0') as i32;

            if i == j.min(s.len()-1) {
                res.push_str(&temp.to_string());
                temp = 0;
                j += k as usize;
            }
        }

        s = res;
    }

    s
}

pub fn main() {
    let s = "11111222223".to_string();
    let k = 3;
    println!("{}", digit_sum(s, k));
}
