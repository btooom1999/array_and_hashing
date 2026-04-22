fn minimum_length(s: String) -> i32 {
    let mut hashmap = [0; 26];
    for b in s.as_bytes() {
        hashmap[(b-b'a') as usize] += 1;
    }

    hashmap.into_iter().fold(0, |acc, num| {
        if num == 0 {
            acc
        } else if num % 2 == 0 {
            acc + 2
        } else {
            acc + 1
        }
    })
}

pub fn main() {
    let s = "abaacbcbb".to_string();
    println!("{}", minimum_length(s));
}
