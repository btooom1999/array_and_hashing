fn are_numbers_ascending(s: String) -> bool {
    let mut min = i32::MIN;
    for word in s.split_whitespace() {
        if let Ok(num) = word.parse::<i32>() {
            if num > min {
                min = num;
            }  else {
                return false;
            }
        }
    }

    true
}

pub fn main() {
    let s = "1 box has 3 blue 4 red 6 green and 12 yellow marbles".to_string();
    println!("{}", are_numbers_ascending(s));
}
