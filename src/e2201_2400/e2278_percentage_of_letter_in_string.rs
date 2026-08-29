fn percentage_letter(s: String, letter: char) -> i32 {
    let mut count = 0;
    for c in s.chars() {
        if c == letter {
            count += 1;
        }
    }

    count * 100 / s.len() as i32
}

pub fn main() {
    let s = "foobar".to_string();
    let letter = 'o';
    println!("{}", percentage_letter(s, letter));
}
