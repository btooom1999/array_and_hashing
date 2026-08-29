use std::collections::HashMap;

fn longest_awesome(s: String) -> i32 {
    let mut hashmap = HashMap::<_, usize>::new();
    hashmap.insert(0, 0);
    let mut bit = 0;
    let mut res = 0;
    for (j, c) in s.chars().enumerate() {
        let num = c as u8 - b'0';
        bit ^= 1 << num;

        let mut count_ones = 0;
        for at in 0..10 {
            if bit >> at & 1 == 1 {
                count_ones += 1;
            }
            if let Some(&i) = hashmap.get(&(bit ^ 1 << at)) {
                res = res.max(j-i);
            }
        }

        if count_ones <= 1 {
            res = res.max(j+1);
        }

        hashmap.entry(bit).or_insert(j);
    }

    res as i32
}

pub fn main() {
    // let s = "185801630663498".to_string();
    let s = "3242415".to_string();
    println!("{}", longest_awesome(s));
}
