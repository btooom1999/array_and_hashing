fn longest_prefix(s: String) -> String {
    let n = s.len();
    let s= s.as_bytes();
    let mut lsp = vec![0; n];
    let mut i = 1;
    let mut len = 0;
    while i < n {
        if s[i] == s[len] {
            len += 1;
            lsp[i] = len;
            i += 1;
        } else if len == 0 {
            i += 1;
        } else {
            len = lsp[len-1];
        }
    }

    String::from_utf8(s[n-lsp[n-1]..].to_vec()).unwrap()
}

pub fn main() {
    let s = "levea".to_string();
    println!("{}", longest_prefix(s));
}
