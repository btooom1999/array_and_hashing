use std::collections::HashMap;

fn max_number_of_balloons(text: String) -> i32 {
    let mut hashmap =
        HashMap::<char, i32>::from([('b', 0), ('a', 0), ('l', 0), ('o', 0), ('n', 0)]);

    for c in text.chars() {
        hashmap.entry(c).and_modify(|v| *v += 1);
    }

    let mut res = i32::MAX;
    for (k, v) in hashmap.into_iter() {
        if k == 'l' || k == 'o' {
            res = std::cmp::min(res, v / 2);
        } else {
            res = std::cmp::min(res, v);
        }
    }

    res
}

pub fn main() {
    let text = "loonbalxballpoon".to_string();
    println!("{}", max_number_of_balloons(text));
}
