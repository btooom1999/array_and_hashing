fn remove_duplicates(s: String, k: i32) -> String {
    let mut stack = Vec::<(u8, usize)>::new();
    for byte in s.as_bytes() {
        if let Some((val, count)) = stack.last_mut() && val == byte {
            *count += 1;

            if *count == k as usize {
                stack.pop();
            }
        } else {
            stack.push((*byte, 1));
        }
    }

    let stack = stack.iter().fold(Vec::new(), |mut acc, &(val, n)| {
        acc.extend(vec![val; n]);
        acc
    });

    String::from_utf8(stack).unwrap()
}

pub fn main() {
    let s = "deeedbbcccbdaa".to_string();
    let k = 3;
    println!("{}", remove_duplicates(s, k));
}
