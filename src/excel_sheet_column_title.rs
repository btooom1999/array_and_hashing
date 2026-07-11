fn convert_to_title(mut column_number: i32) -> String {
    let mut res = Vec::new();
    while column_number > 0 {
        for i in (1..27).rev() {
            if (column_number-i)%26 == 0 {
                column_number = (column_number - i) / 26;
                res.push((i-1) as u8 + b'A');
                break;
            }
        }
    }

    res.reverse();
    String::from_utf8(res).unwrap()
}

pub fn main() {
    let column_number = 18250;
    println!("{}", convert_to_title(column_number));
}
