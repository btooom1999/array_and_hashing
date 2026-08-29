fn reverse_words(s: String) -> String {
    let mut s = s.chars().collect::<Vec<_>>();
    s.push(' ');

    let mut l = 0;
    for j in 0..s.len() - 1 {
        if !s[j].is_whitespace() {
            if j > 0 && s[j-1].is_whitespace() {
                l = j;
            }

            if s[j+1].is_whitespace() {
                let mut r = j;
                while l < r {
                    (s[l], s[r]) = (s[r], s[l]);
                    l += 1;
                    r -= 1;
                }
            }
        }
    }

    s.pop();
    s.iter().collect::<String>()
}

pub fn main() {
    let s = "I love u".to_string();
    println!("{}", reverse_words(s));
}
