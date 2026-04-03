use std::collections::HashMap;

fn digit_count(num: String) -> bool {
    let mut hashmap = HashMap::<i32, i32>::new();
    for num in num.chars() {
        *hashmap.entry((num as u8 - b'0') as i32).or_default() += 1;
    }

    for (i, num) in num.chars().enumerate() {
        let i = i as i32;
        let num = (num as u8 - b'0') as i32;
        if *hashmap.get(&i).unwrap_or(&0) != num {
            return false;
        }
    }

    true
}

pub fn main() {
    let num = "1210".to_string();
    println!("{}", digit_count(num));
}
