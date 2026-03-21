use std::collections::{HashMap, HashSet};

fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
    let mut words = HashSet::new();
    let mut prefix = HashSet::new();

    for word in &dictionary {
        words.insert(word.to_owned());

        if prefix.contains(word) {
            continue;
        }

        for i in 1..=word.len() {
            prefix.insert(word[..i].to_string());
        }
    }

    let mut max = 0;
    let s = s.as_bytes();
    let mut hashmap = HashMap::<Vec<(String, i32)>, i32>::new();
    for (i, &byte) in s.iter().enumerate() {
        let mut j = i;
        while j < s.len() {
            let str = String::from_utf8(s[i..=j].into()).unwrap();
            if !prefix.contains(&str) { break; }

            if words.contains(&str) {
                let mut temp_hashmap = hashmap.clone();
                hashmap.clear();

                if temp_hashmap.is_empty() {
                    hashmap.insert(vec![(str, j as i32)], (j-i+1) as i32);
                    continue;
                }

                for (key, value) in temp_hashmap.into_iter() {
                    let mut key = key.clone();
                    let mut value = value;

                    let last = key.last().unwrap().clone();
                    if last.1 >= i as i32 {
                        hashmap.insert(key.clone(), value);
                        key.pop();
                        value = std::cmp::max(0, value - last.0.len() as i32);
                    }

                    key.push((str.clone(), j as i32));
                    value += (j-i+1) as i32;
                    hashmap.insert(key, value);
                    max = std::cmp::max(max, value);
                }
            }

            j += 1;
        }
    }

    s.len() as i32 - max
}

pub fn main() {
    let s = "leetscode".to_string();
    let dictionary = ["leet","code", "leetcode"].into_iter().map(String::from).collect::<Vec<_>>();
    // let s = "metzeaencgpgvsckjrqafkxgyzbe".to_string();
    // let dictionary = ["zdzz","lgrhy","r","ohk","zkowk","g","zqpn","anoni","ka","qafkx","t","jr","xdye","mppc","bqqb","encgp","yf","vl","ctsxk","gn","cujh","ce","rwrpq","tze","zxhg","yzbe","c","o","hnk","gv","uzbc","xn","kk","ujjd","vv","mxhmv","ugn","at","kumr","ensv","x","uy","gb","ae","jljuo","xqkgj"].into_iter().map(String::from).collect::<Vec<_>>();

    println!("{}", min_extra_char(s, dictionary));
}
