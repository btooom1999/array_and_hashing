fn int_to_roman(num: i32) -> String {
    let mut romans = vec![
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ];

    let nums = num.to_string();
    let n = nums.len();
    let nums = nums.as_bytes();
    let mut res = String::new();
    for i in 0..n {
        let mut num = (nums[i]-b'0') as i32 * 10i32.pow((n-i-1) as u32);
        while num > 0 {
            match num {
                4 | 9 | 40 | 90 | 400 | 900 => {
                    match num {
                        4 => res.push_str("iv"),
                        9 => res.push_str("ix"),
                        40 => res.push_str("XL"),
                        90 => res.push_str("XC"),
                        400 => res.push_str("CD"),
                        900 => res.push_str("CM"),
                        _ => unreachable!(),
                    }
                    break;
                }
                _ => {
                    while let Some(last) = romans.last() && num >= last.1 {
                        res.push(last.0);
                        num -= last.1;
                    }
                    romans.pop();
                }
            }
        }
    }

    res
}

pub fn main() {
    let num = 1994;
    println!("{}", int_to_roman(num));
}
