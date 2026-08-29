use std::collections::HashMap;

fn kth_distinct(arr: Vec<String>, k: i32) -> String {
    let mut hashmap = HashMap::with_capacity(arr.len());

    for val in &arr {
        hashmap.entry(val).and_modify(|v| *v += 1).or_insert(1);
    }

    let mut count = 0;
    for val in &arr {
        if let Some(num) = hashmap.get(val)
            && *num == 1
        {
            count += 1;
        }

        if count == k {
            return val.to_owned();
        }
    }

    String::new()
}

pub fn main() {
    let arr = vec![
        "d".to_string(),
        "b".to_string(),
        "c".to_string(),
        "b".to_string(),
        "c".to_string(),
        "a".to_string(),
    ];
    let k = 2;

    println!("{}", kth_distinct(arr, k))
}
