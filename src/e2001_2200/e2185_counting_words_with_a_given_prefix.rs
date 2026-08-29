fn prefix_count(words: Vec<String>, pref: String) -> i32 {
    let mut count = 0;
    for word in words {
        if word.starts_with(pref.as_str()) {
            count += 1;
        }
    }

    count
}

pub fn main() {
    let words = ["pay","attention","practice","attend"].into_iter().map(String::from).collect::<Vec<_>>();
    let pref = "at".to_string();
    println!("{}", prefix_count(words, pref));
}
