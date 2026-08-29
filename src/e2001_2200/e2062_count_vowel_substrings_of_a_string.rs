const MOD: i32 = 1_000_000_007;

fn count_vowel_substrings(word: String) -> i32 {
    let word = word.as_bytes();
    let n = word.len();
    let mut res = 0;
    for x in 0..n {
        let (mut a, mut e, mut i, mut o, mut u) = (0, 0, 0, 0, 0);
        for w in &word[x..] {
            match w {
                b'a' => a = 1,
                b'e' => e = 1,
                b'i' => i = 1,
                b'o' => o = 1,
                b'u' => u = 1,
                _ => break,
            }
            if a+e+i+o+u == 5 {
                res = (res + 1) % MOD;
            }
        }
    }

    res
}

pub fn main() {
    let word = "aeiouu".to_string();
    println!("{}", count_vowel_substrings(word));
}
