fn longest_substring(s: String, k: i32) -> i32 {
    if k > s.len() as i32 {
        return 0;
    }

    let mut res = 0;
    let s = s.as_bytes();
    let n = s.len();
    for i in (0..n).rev() {
        let mut hashmap = [0;26];
        let mut count = 0;
        for j in i..n {
            let idx = (s[j]-b'a') as usize;
            hashmap[idx] += 1;

            if hashmap[idx] == 1 {
                count += 1;
            }

            if hashmap[idx] == k {
                count -= 1;
            }

            if count == 0 {
                res = res.max(j-i+1);
            }
        }
    }

    res as i32
}

pub fn main() {
    let s = "ababbc".to_string();
    let k = 2;
    println!("{}", longest_substring(s, k));
}
