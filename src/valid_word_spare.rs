fn valid_word_spare(words: Vec<String>) -> bool {
    let words = words.iter().map(|v| v.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    for i in 0..words.len() {
        for j in 0..words.len() {
            if words[i][j] != words[j][i] {
                return false;
            }
        }
    }

    true
}

pub fn main() {
    let words = vec!["ball", "area", "read", "lady"].iter().map(|v| v.to_string()).collect::<Vec<_>>();
    // let words = vec!["ball", "area", "read", "lady"].iter().map(|v| v.to_string()).collect::<Vec<_>>();
    println!("{}", valid_word_spare(words))
}
