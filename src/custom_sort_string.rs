use std::collections::HashMap;

fn custom_sort_string(order: String, s: String) -> String {
    let mut hashmap = HashMap::<char, i32>::with_capacity(s.len());
    for c in s.chars() {
        *hashmap.entry(c).or_default() += 1;
    }

    let mut res = Vec::new();
    for c in order.chars() {
        if let Some(n) = hashmap.get(&c) {
            res.append(&mut vec![c; *n as usize]);
            hashmap.remove(&c);
        }
    }

    let mut exceed_res = hashmap.into_iter().fold(Vec::new(), |mut vec, x| {
        vec.append(&mut vec![x.0; x.1 as usize]);
        vec
    });
    exceed_res.sort();

    res.append(&mut exceed_res);

    String::from_iter(res)
}

pub fn main() {
    let order = "bcafg".to_string();
    let s = "abczmbdbdbdzm".to_string();
    println!("{}", custom_sort_string(order, s));
}
