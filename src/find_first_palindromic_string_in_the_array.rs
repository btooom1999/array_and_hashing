fn first_palindrome(words: Vec<String>) -> String {
    for w in &words {
        let chars = w.chars().collect::<Vec<_>>();
        let mut l = 0;
        let mut r = w.len();
        let mut valid = true;
        while l < r {
            r -= 1;
            if chars[l] != chars[r] {
                valid = false;
                break;
            }
            l += 1;
        }
        if valid {
            return w.to_owned();
        }
    }

    String::new()
}

pub fn main() {
    let words = vec!["abc", "car", "ada", "racecar", "cool"]
        .into_iter()
        .map(String::from)
        .collect::<Vec<_>>();
    println!("{}", first_palindrome(words));
}
