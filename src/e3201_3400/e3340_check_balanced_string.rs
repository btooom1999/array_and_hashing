fn is_balanced(num: String) -> bool {
    let num = num.as_bytes();
    let mut res = [0;2];

    for i in 0..num.len() {
        res[i%2] += num[i] - b'0';
    }

    res[0] == res[1]
}

pub fn main() {
    let num = "24123".to_string();
    println!("{:?}", is_balanced(num));
}
