 fn reverse(mut x: i32) -> i32 {
    let stringify_num = x.to_string();
    let mut stringify_num = stringify_num.chars().rev().collect::<String>();
    let mut is_negative = false;

    if stringify_num.as_bytes()[stringify_num.len() - 1] == b'-' {
        stringify_num.pop();
        stringify_num = format!("-{}", stringify_num);
        is_negative = true;
    }

    if is_negative {
        let min = i32::MIN.to_string();
        if stringify_num.len() > min.len() {
            return 0;
        }
        if stringify_num.len() < min.len() {
            return stringify_num.parse::<i32>().unwrap();
        }
        for (val1, val2) in stringify_num.chars().skip(1).zip(min.chars().skip(1)) {
            let val1 = (val1 as u8 - b'0') as i32;
            let val2 = (val2 as u8 - b'0') as i32;
            if val1 > val2 {
                return 0;
            } else if val1 < val2 {
                return stringify_num.parse::<i32>().unwrap();
            }
        }

        return stringify_num.parse::<i32>().unwrap();
    }

    let max = i32::MAX.to_string();
    if stringify_num.len() > max.len() {
        return 0;
    }
    if stringify_num.len() < max.len() {
        return stringify_num.parse::<i32>().unwrap();
    }
    for (val1, val2) in stringify_num.chars().zip(max.chars()) {
        let val1 = (val1 as u8 - b'0') as i32;
        let val2 = (val2 as u8 - b'0') as i32;
        if val1 > val2 {
            return 0;
        } else if val1 < val2 {
            return stringify_num.parse::<i32>().unwrap();
        }
    }

    stringify_num.parse::<i32>().unwrap()
}

pub fn main() {
    // let x = 123;
    let x = i32::MIN;
    println!("{}", reverse(x));
}
