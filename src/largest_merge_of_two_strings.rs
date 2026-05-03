fn largest_merge(word1: String, word2: String) -> String {
    let word1 = word1.as_bytes();
    let word2 = word2.as_bytes();
    let mut i = 0;
    let mut j = 0;

    let mut res = Vec::new();
    while i < word1.len() && j < word2.len() {
        if word1[i..] > word2[j..] {
            res.push(word1[i]);
            i += 1;
        } else {
            res.push(word2[j]);
            j += 1;
        }
    }

    res.extend(word1[i..].to_vec());
    res.extend(word2[j..].to_vec());

    String::from_utf8(res).unwrap()
}

// urr
// urrru

pub fn main() {
    // let word1 = "abcabc".to_string();
    // let word2 = "abdcaba".to_string();
    let word1 = "b".to_string();
    let word2 = "bzz".to_string();
    println!("{}", largest_merge(word1, word2));
}
