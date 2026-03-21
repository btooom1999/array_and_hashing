fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
    let mut sentence = sentence.split(" ").collect::<Vec<_>>();

    let mut i = 0;
    while i < sentence.len() {
        let w = sentence[i];
        let mut min = w;
        for dict in &dictionary {
            if w.starts_with(dict) && dict.len() < min.len() {
                min = dict;
            }
        }
        sentence[i] = min;
        i += 1;
    }

    sentence.join(" ")
}

pub fn main() {
    let dictionary = ["cat","bat","rat"].into_iter().map(String::from).collect::<Vec<_>>();
    let sentence = "the cattle was rattled by the battery".to_string();
    println!("{}", replace_words(dictionary, sentence));
}
