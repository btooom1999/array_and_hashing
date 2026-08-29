use std::collections::HashMap;

fn longest_wpi(hours: Vec<i32>) -> i32 {
    let n = hours.len();
    let mut hashmap = HashMap::new();

    let mut res = 0;
    let mut prefix = 0;
    for j in 0..n {
        prefix += if hours[j] > 8 { 1 } else { -1 };
        if prefix > 0 {
            res = res.max((j+1) as i32);
        } else if let Some(&i) = hashmap.get(&(prefix-1)) {
            res = res.max((j-i) as i32);
        }
        hashmap.entry(prefix).or_insert(j);
    }

    res
}

pub fn main() {
    let hours = [0,0,0,9,9,9].to_vec();
    println!("{}", longest_wpi(hours));
}
