use std::collections::BinaryHeap;

fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
    let mut chars = [0; 26];
    for b in s.as_bytes() {
        chars[(b-b'a') as usize] += 1;
    }

    let mut chars = chars.into_iter().enumerate().map(|v| (v.0 as u8 + b'a', v.1)).filter(|v| v.1 > 0).collect::<BinaryHeap<_>>();
    let mut res = Vec::new();
    let mut queue = Vec::new();
    while let Some((byte1, mut count)) = chars.pop() {
        if let Some((byte2, mut amount)) = queue.pop() {
            while count > 0 && amount > 0 {
                res.push(byte1);
                let n = std::cmp::min(amount, repeat_limit);
                res.extend(vec![byte2; n as usize]);
                amount -= n;
                count -= 1;
            }

            if amount > 0 {
                queue.push((byte2, amount));
            }
        }

        let n = std::cmp::min(count, repeat_limit);
        count -= n;
        res.extend(vec![byte1; n as usize]);
        if count > 0 {
            queue.push((byte1, count));
        }
    }

    String::from_utf8(res).unwrap()
}

pub fn main() {
    let s = "cczazcc".to_string();
    let repeat_limit = 3;
    println!("{}", repeat_limited_string(s, repeat_limit));
}
