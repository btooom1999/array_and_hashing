use std::collections::HashMap;

fn count_largest_group(n: i32) -> i32 {
    let mut hashmap = HashMap::<i32, i32>::new();
    for num in 1..n+1 {
        let num = num.to_string();
        let mut key = 0;
        for b in num.as_bytes() {
            key += (b-b'0') as i32;
        }
        *hashmap.entry(key).or_default() += 1;
    }

    let mut count = 0;
    let mut max = 0;
    for &maximum in hashmap.values() {
        if maximum > max {
            count = 1;
            max = maximum;
        } else if maximum == max {
            count += 1;
        }
    }

    count
}

pub fn main() {
    let n = 13;
    println!("{}", count_largest_group(n));
}
