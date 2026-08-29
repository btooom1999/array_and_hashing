fn make_strings_equal(s: String, target: String) -> bool {
    s.contains('1') == target.contains('1')
}

pub fn main() {
    let s = "1010".to_string();
    let target = "0110".to_string();
    println!("{}", make_strings_equal(s, target));
}
