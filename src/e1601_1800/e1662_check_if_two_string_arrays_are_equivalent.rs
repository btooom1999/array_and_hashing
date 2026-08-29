fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
    let mut str1 = String::new();
    let mut str2 = String::new();

    for i in 0..std::cmp::max(word1.len(), word2.len()) {
        if let Some(s1) = word1.get(i) {
            str1.push_str(s1);
        }

        if let Some(s2) = word2.get(i) {
            str2.push_str(s2);
        }
    }

    str1 == str2
}

pub fn main() {
    let word1 = ["ab", "c"].into_iter().map(String::from).collect::<Vec<_>>();
    let word2 = ["a", "bc"].into_iter().map(String::from).collect::<Vec<_>>();
    println!("{}", array_strings_are_equal(word1, word2));
}
