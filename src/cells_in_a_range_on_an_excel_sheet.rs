fn cells_in_range(s: String) -> Vec<String> {
    let mut s = s.split(':');
    let first = s.next().unwrap();
    let second = s.next().unwrap();

    let first_byte = first.as_bytes()[0..1][0];
    let first_num = first[1..].parse::<i32>().unwrap();
    let second_byte = second.as_bytes()[0..1][0];
    let second_num = second[1..].parse::<i32>().unwrap();

    let mut res = Vec::new();
    for byte in first_byte..=second_byte {
        for num in first_num..=second_num {
            res.push(format!("{}{}", byte as char, num));
        }
    }

    res

}

pub fn main() {
    let s = "K1:L2".to_string();
    println!("{:?}", cells_in_range(s));
}
