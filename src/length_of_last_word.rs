fn length_of_last_word(s: String) -> i32 {
    let mut len = 0;

    for i in (0..s.len()).rev() {
        if s.chars().nth(i).unwrap() == ' ' {
            if i < s.len() - 1 && s.chars().nth(i+1).unwrap() != ' ' {
                break;
            }

            continue;
        }

        len += 1;
    }

    len
}

pub fn main() {
    let s = "Hello World".to_string();

    println!("{}", length_of_last_word(s));
}
