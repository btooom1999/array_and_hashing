use std::collections::HashMap;

fn find_the_longest_substring(s: String) -> i32 {
    let mut hashmap = HashMap::<String, i32>::from([]);
    let mut vowels = [('a', 0), ('e', 0), ('i', 0), ('o', 0), ('u', 0)];
    let mut res = 0;
    for (l, char) in s.chars().enumerate() {
        if let Some(val) = vowels.iter_mut().find(|v| v.0 == char) {
            *val = (val.0, val.1 + 1);
        }

        let str = vowels
            .iter()
            .filter_map(|(c, v)| if v % 2 == 0 { None } else { Some((*c, *v % 2)) })
            .fold(String::new(), |mut s, (c, n)| {
                s.push_str(&c.to_string().repeat(n));
                s
            });

        if str.is_empty() {
            res = std::cmp::max(res, l as i32 + 1);
        } else if let Some(val) = hashmap.get(&str) {
            res = std::cmp::max(res, l as i32 + 1 - *val);
        } else {
            hashmap.insert(str, l as i32 + 1);
        }
    }

    res
}

pub fn main() {
    let s = "eleetminicoworoep".to_string();
    // let s = "leetcodeisgreat".to_string();
    // let s = "bcbcbc".to_string();
    // let s = "bcbabc".to_string();
    // let s = "aeiaaioooaauuaeiou".to_string();

    println!("{}", find_the_longest_substring(s));
}
