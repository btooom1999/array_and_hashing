fn max_product(words: Vec<String>) -> i32 {
    let mut masks = vec![-1; words.len()];
    for (i, word) in words.iter().enumerate() {
        let mut mask = 0;
        for &b in word.as_bytes() {
            mask |= 1 << (b-b'a') as i32;
        }

        masks[i] = mask;
    }

    let mut res = 0;
    for i in 0..masks.len() {
        for j in i+1..masks.len() {
            if words[i].len() * words[j].len() <= res {
                continue;
            }

            if masks[i] & masks[j] == 0 {
                res = res.max(words[i].len() * words[j].len());
            }
        }
    }

    res as i32
}

pub fn main() {
    let words = ["a","ab","abc","d","cd","bcd","abcd"].into_iter().map(String::from).collect();
    // let words = ["abcw","baz","foo","bar","xtfn","abcdef"].into_iter().map(String::from).collect();
    println!("{:?}", max_product(words));
}
