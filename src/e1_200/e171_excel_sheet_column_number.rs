fn title_to_number(column_title: String) -> i32 {
    let mut res = 0;
    for &b in column_title.as_bytes() {
        res *= 26;
        res += (b - b'A' + 1) as i32;
    }

    res
}

pub fn main() {
    let column_title = "ZY".to_string();
    println!("{}", title_to_number(column_title));
}
