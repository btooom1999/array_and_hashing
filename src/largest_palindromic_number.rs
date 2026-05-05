fn largest_palindromic(num: String) -> String {
    let mut hashmap = [0; 10];
    for c in num.chars() {
        hashmap[(c as u8 - b'0') as usize] += 1;
    }

    let mut left = Vec::new();
    let mut right = Vec::new();
    let mut odd = usize::MAX;
    for idx in (0..10).rev() {
        let mut count = hashmap[idx];
        if count % 2 == 1 {
            if odd == usize::MAX { odd = idx };
            count -= 1;
        }

        left.extend(vec![idx as u8 + b'0'; count/2]);
        right.extend(vec![idx as u8 + b'0'; count/2]);
    }

    if odd != usize::MAX {
        left.push(odd as u8 + b'0');
    }

    right.reverse();
    left.extend(right);

    let mut i = 0;
    let mut j = left.len()-1;
    while left[i] == b'0' && i < j {
        i += 1;
        j -= 1;
    }

    String::from_utf8(left[i..=i.max(j)].to_vec()).unwrap()
}

pub fn main() {
    let num = "000000".to_string();
    println!("{}", largest_palindromic(num));
}
