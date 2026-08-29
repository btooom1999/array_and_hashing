use std::collections::HashMap;

fn num_splits(s: String) -> i32 {
    let mut res = 0;
    let mut hashmap = HashMap::<_, i32>::new();

    for c in s.chars() {
        *hashmap.entry(c).or_default() += 1;
    }

    let mut temp = HashMap::<_, i32>::new();
    for c in s.chars() {
        *temp.entry(c).or_default() += 1;
        let val = hashmap.get_mut(&c).unwrap();
        if *val == 1 {
            hashmap.remove(&c);
        } else {
            *val -= 1;
        }

        if temp.len() == hashmap.len() {
            res += 1;
        }
    }

    res
}

pub fn main() {
    let s = "aacaba".to_string();
    println!("{}", num_splits(s));
}
