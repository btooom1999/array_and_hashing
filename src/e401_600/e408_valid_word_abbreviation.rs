
fn valid_word_abbreviation(word: String, abbr: String) -> bool {
    let mut i = 0;
    let mut j = 0;
    let word = word.chars().collect::<Vec<_>>();
    let abbr = abbr.chars().collect::<Vec<_>>();

    while i < word.len() && j < abbr.len() {
        if word[i] == abbr[j] {
            i += 1;
            j += 1;
        } else if !abbr[j].is_ascii_digit() || abbr[j] == '0' {
            return false;
        } else {
            let mut sub = 0;
            while abbr[j].is_ascii_digit() {
                sub = sub * 10 + abbr[j].to_digit(10).unwrap() as usize;
                j += 1;
            }
            i += sub;
        }
    }

    i == word.len() && j == word.len()
}

pub fn main() {
    let word = "localization".to_string();
    let abbr = "l10n".to_string();
    println!("{}", valid_word_abbreviation(word, abbr));
}
