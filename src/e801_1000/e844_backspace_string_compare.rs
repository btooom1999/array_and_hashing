fn backspace_compare(s: String, t: String) -> bool {
    let s = s.as_bytes();
    let t = t.as_bytes();

    let mut i = s.len() as i32 - 1;
    let mut j = t.len() as i32 - 1;

    let mut count1 = 0;
    let mut count2 = 0;

    loop {
        if i < 0 && j < 0 {
            return true;
        }

        if (i < 0 && j >= 0 && t[j as usize] != b'#') || (i >= 0 && j < 0 && s[i as usize] != b'#') {
            return false;
        }

        if i >= 0 && j >= 0 && s[i as usize] != b'#' && t[j as usize] != b'#' && s[i as usize] != t[j as usize] {
            return false;
        }

        if i >= 0 && s[i as usize] == b'#' {
            while i >= 0 && s[i as usize] == b'#' {
                count1 += 1;
                i -= 1;
            }

            while i >= 0 && count1 > 0 && s[i as usize] != b'#' {
                i -= 1;
                count1 -= 1;
            }
        } else if j >= 0 && t[j as usize] == b'#' {
            while j >= 0 && t[j as usize] == b'#' {
                count2 += 1;
                j -= 1;
            }

            while j >= 0 && count2 > 0 && t[j as usize] != b'#' {
                j -= 1;
                count2 -= 1;
            }
        } else {
            i -= 1;
            j -= 1;
        }
    }
}

pub fn main() {
    let s = "bxj##tw".to_string();
    let t = "bxo#j##tw".to_string();
    println!("{}", backspace_compare(s, t));
}
