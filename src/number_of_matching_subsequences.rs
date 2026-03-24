use std::collections::HashMap;

fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
    let mut hashmap = HashMap::new();;

    let mut count = 0;
    let s = s.as_bytes();
    for w in words {
        let byte = w.as_bytes();
        let mut i = 0;
        let mut j = 0;
        if let Some(&flag) = hashmap.get(&w) {
            if flag {
                count += 1;
            }

            continue;
        }

        loop {
            match (s.get(i), byte.get(j)) {
                (Some(val1), Some(val2)) => {
                    if val1 == val2 {
                        i += 1;
                        j += 1;
                    } else {
                        i += 1;
                    }
                }
                (Some(_), _) | (None, None) => {
                    count += 1;
                    hashmap.insert(w, true);
                    break;
                }
                _ => {
                    hashmap.insert(w, false);
                    break;
                }
            }
        }
    }

    count
}

pub fn main() {
    let s = "abcde".to_string();
    let words = ["a","bb","acd","ace"].into_iter().map(String::from).collect::<Vec<_>>();
    println!("{}", num_matching_subseq(s, words));
}
