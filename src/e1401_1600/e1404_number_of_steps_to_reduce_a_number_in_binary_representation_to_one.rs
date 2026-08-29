fn num_steps(mut s: String) -> i32 {
    let mut res = 0;
    let mut carry = 0;
    while s.len() > 1 && s[..s.len()-1].contains("1") {
        let mut c = s.pop().unwrap();
        if carry == 1 {
            c = if c == '1' {
                '0'
            } else {
                carry = 0;
                '1'
            };
        }

        if c == '1' {
            carry = 1;
            res += 2;
        } else {
            res += 1;
        }
    }

    res+carry
}

pub fn main() {
    let s = "1101".to_string();
    println!("{}", num_steps(s));
}
