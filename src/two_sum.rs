use std::collections::HashMap;

pub fn main() {
    let nums = [2, 7, 11, 15];
    let target = 9;
    let mut result: Option<(usize, usize)> = None;

    let mut hashmap: HashMap<i32, usize> = HashMap::new();

    for (i, num) in nums.iter().enumerate() {
        let value = target - num;
        if let Some(prev_i) = hashmap.get(num) {
            result = Some((prev_i.to_owned(), i));
            break;
        } else {
            hashmap.insert(value, i);
        }
    }

    println!("result: {:?}", result);
}
