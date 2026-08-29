fn is_prefix_and_suffix(a: String, b: String) -> bool {
    b.starts_with(a.as_str()) && b.ends_with(a.as_str())
}

fn count_prefix_suffix_pairs(words: Vec<String>) -> i32 {
    let mut count = 0;

    for i in 0..words.len() {
        for j in i+1..words.len() {
            if is_prefix_and_suffix(words[i].clone(), words[j].clone()) {
                count += 1;
            }
        }
    }

    count
}

pub fn main() {
    let words = ["ab","abab"].into_iter().map(String::from).collect::<Vec<_>>();
    println!("{}", count_prefix_suffix_pairs(words));
}
