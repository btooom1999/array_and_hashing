fn minimum_pushes(word: String) -> i32 {
    let mut chars= [0; 26];
    for b in word.as_bytes() {
        chars[(b-b'a') as usize] += 1;
    }

    chars.sort_unstable();
    let mut res = 0;
    let mut amount = 1;
    for i in (0..chars.len()).rev() {
        res += (amount as f32 / 8f32).ceil() as i32 * chars[i];
        amount += 1;
    }

    res
}

pub fn main() {
    let word = "abhrlngxyjkezwcm".to_string();
    println!("{}", minimum_pushes(word));
}
