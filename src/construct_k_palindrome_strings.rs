fn can_construct(s: String, k: i32) -> bool {
    let k = k as usize;
    if s.len() < k {
        return false;
    }

    let mut hashmap = [0;26];
    for b in s.as_bytes() {
        let k = (b-b'a') as usize;
        hashmap[k] += 1;
    }

    let mut odd_count = 0;
    for count in hashmap {
        if count % 2 != 0 {
            odd_count += 1;
        }
    }

    odd_count <= k
}

pub fn main() {
    let s = "truem".to_string();
    let k = 4;
    println!("{}", can_construct(s, k));
}
