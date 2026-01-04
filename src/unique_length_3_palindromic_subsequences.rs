use std::collections::HashSet;

fn count_palindromic_subsequence(s: String) -> i32 {
    let s = s.as_bytes();
    let mut hashset = HashSet::<String>::new();
    let mut words = [(-1, 0); 26]; // (index, count)
    for (j, c) in s.iter().enumerate() {
        let k = (c - b'a') as usize;
        if words[k].0 != -1 {
            let i = words[k].0 as usize;
            let mut valid = false;
            for x in (i + 1)..j {
                valid = true;
                hashset.insert(format!("{}{}{}", s[i] as char, s[x] as char, *c as char));
            }

            words[k].1 += 1;
            if words[k].1 == 3 {
                hashset.insert((*c as char).to_string().repeat(words[k].1));
            }

            if valid {
                words[k].0 = j as i32;
            }
        } else {
            words[k] = (j as i32, words[k].1 + 1);
        }
    }

    hashset.len() as i32
}

pub fn main() {
    let s = "abbccbabac".to_string();
    println!("{}", count_palindromic_subsequence(s));
}
