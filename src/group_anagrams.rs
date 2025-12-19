use std::collections::HashMap;

fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut anagrams: HashMap<[u8; 26], Vec<String>> = HashMap::with_capacity(strs.len());

    for str in strs.into_iter() {
        let mut key = [0u8; 26];
        for c in str.as_bytes() {
            key[(c - b'a') as usize] += 1;
        }
        anagrams.entry(key).or_insert_with(Vec::new).push(str);
    }

    anagrams.into_values().collect()
}

pub fn main() {
    // let strs = vec!["eat".to_string(), "tea".to_string(), "tan".to_string(), "ate".to_string(), "nat".to_string(), "bat".to_string()];
    let strs= vec!["sat".to_string(),"lea".to_string(),"arm".to_string(),"sin".to_string(),"the".to_string(),"nod".to_string(),"guy".to_string(),"ins".to_string(),"rod".to_string()];


    let result = group_anagrams(strs);

    println!("{:?}", result);
}
