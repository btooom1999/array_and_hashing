fn reverse_words(s: String) -> String {
    let mut n = s.len();
    let mut s = s.into_bytes();
    let mut flag = false;
    let mut i = 0;
    let mut j = 0;

    while j < n-1 {
        if s[j].is_ascii_whitespace() && (s[j+1] == s[j] || j == 0) {
            if j != 0 { i += 1; }
            flag = true;
            break;
        }

        i += 1;
        j += 1;
    }

    while flag && i < n {
        while j < n && s[j].is_ascii_whitespace() {
            j += 1;
        }

        while j < n && s[j].is_ascii_alphanumeric() {
            s[i] = s[j];
            s[j] = b' ';
            j += 1;
            i += 1;
        }

        s[i] = b' ';
        i += 1;
    }

    for i in (0..n).rev() {
        if !s[i].is_ascii_whitespace() { break; }
        n -= 1;
    }

    s.truncate(n);
    s.reverse();

    let mut i = 0;
    while i < n {
        let mut j = i;
        while j < n-1 && !(s[j].is_ascii_alphanumeric() && s[j+1].is_ascii_whitespace()) {
            j += 1;
        }

        let next = j+2;
        while i < j {
            (s[i], s[j]) = (s[j], s[i]);
            i += 1;
            j -= 1;
        }

        i = next;
    }

    String::from_utf8(s).unwrap()
}

pub fn main() {
    let s = "  hello   world   ".to_string();
    println!("{}", reverse_words(s));
}
