fn is_circular_sentence(sentence: String) -> bool {
    let sentence = sentence.split(" ").collect::<Vec<&str>>();

    for i in 0..sentence.len() {
        if i == sentence.len() - 1
            && sentence[i].chars().nth(sentence[i].len() - 1) != sentence[0].chars().next()
        {
            return false;
        }

        if i != sentence.len() - 1
            && sentence[i].chars().nth(sentence[i].len() - 1) != sentence[i + 1].chars().next()
        {
            return false;
        }
    }

    true
}

pub fn main() {
    let sentence = "Leetcode eisc cool".to_string();
    println!("{}", is_circular_sentence(sentence));
}
