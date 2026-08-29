fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
    let sentence1 = sentence1.split(' ').collect::<Vec<_>>();
    let sentence2 = sentence2.split(' ').collect::<Vec<_>>();
    let mut i = 0;
    let mut j = 0;
    let mut skip = 1;
    let (smallest, largest) = if sentence1.len() > sentence2.len() { (sentence2, sentence1) } else { (sentence1, sentence2) };
    let len1 = largest.len();
    let len2 = smallest.len();
    while j < len2 && i < len1 {
        if largest[i] != smallest[j] {
            if skip == 0 {
                return false;
            }

            skip -= 1;

            let mut r = len1 - 1;
            let mut temp_i = -1;
            while i < r {
                if largest[r] == smallest[j] {
                    temp_i = r as i32;
                    break;
                }
                r -= 1;
            }
            if temp_i != -1 {
                i = temp_i as usize;
            } else {
                i = len1;
            }
        } else {
            i += 1;
            j += 1;
        }
    }

    if i == len1 && j < len2 {
        return false;
    }

    if j == len2 && i < len1 {
        return skip == 1;
    }

    true
}

pub fn main() {
    let sentence1 = "A".to_string();
    let sentence2 = "a A a A".to_string();
    println!("{}", are_sentences_similar(sentence1, sentence2));
}
