fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
    let mut res = Vec::new();
    for num in left..=right {
        if num.to_string().chars().all(|v| v != '0' && num % (v as u8 - b'0') as i32 == 0) {
            res.push(num);
        }
    }

    res
}

pub fn main() {
    let left = 1;
    let right = 22;
    println!("{:?}", self_dividing_numbers(left, right));
}
