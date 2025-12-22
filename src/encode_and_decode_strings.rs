fn encode_and_decode_strings(strings: Vec<String>) -> Vec<String> {
    let mut encoded_strings = String::new();

    for s in strings.into_iter() {
        encoded_strings += &format!("{}#{}", s.len(), s);
    }

    println!("encoded_string: {}", encoded_strings);

    let mut decoded_strings: Vec<String> = Vec::new();
    let mut i: usize = 0;
    let mut from_number: usize = 0;

    while encoded_strings.chars().nth(from_number).is_some()
        && let Some(c) = encoded_strings.chars().nth(i)
    {
        if c != '#' {
            i += 1;
            continue;
        }

        let number = encoded_strings[from_number..i].parse::<usize>().unwrap();
        let s = encoded_strings[i + 1..i + number + 1].to_string();
        decoded_strings.push(s.clone());

        from_number = i + number + 1;
        i += 1;
    }

    println!("decode_strings: {:?}", decoded_strings);

    decoded_strings
}

pub fn main() {
    // let strings = vec!["Hello", "world"].iter().map(|s| s.to_string()).collect();
    let strings = ["abc", "!@"].iter().map(|s| s.to_string()).collect();

    encode_and_decode_strings(strings);
}
