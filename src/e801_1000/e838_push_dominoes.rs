fn push_dominoes(dominoes: String) -> String {
    let mut dominoes = dominoes.into_bytes();
    let mut left = Vec::new();
    let mut right = Vec::new();

    for j in 0..dominoes.len() {
        if dominoes[j] == b'R' {
            right.push(j+1);
        } else if dominoes[j] == b'L' {
            if let Some(mut i) = right.pop() {
                let mut j = j-1;
                while i < j && (dominoes[i] == b'.' || dominoes[j] == b'.') {
                    if dominoes[i] == b'.' {
                        dominoes[i] = b'R';
                        i += 1;
                    }
                    if dominoes[j] == b'.' {
                        dominoes[j] = b'L';
                        j -= 1;
                    }
                }
            } else {
                left.push(j.wrapping_sub(1));
            }
        }
    }

    while let Some(mut j) = left.pop() {
        while dominoes.get(j).is_some_and(|&v| v == b'.') {
            dominoes[j] = b'L';
            j = j.wrapping_sub(1);
        }
    }

    while let Some(mut i) = right.pop() {
        while dominoes.get(i).is_some_and(|&v| v == b'.') {
            dominoes[i] = b'R';
            i += 1;
        }
    }

    String::from_utf8(dominoes).unwrap()
}

pub fn main() {
    let dominoes = ".........L.L".to_string();
    println!("{}", push_dominoes(dominoes));
}
