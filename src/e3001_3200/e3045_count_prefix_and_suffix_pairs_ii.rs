const MOD: i64 = 2i64.pow(45)-1;
const BASE: i64 = 26;

#[derive(Debug)]
struct Trie {
    children: [Option<Box<Trie>>; 26],
    count: i64,
}

impl Trie {
    fn new() -> Self {
        const NONE: Option<Box<Trie>> = None;
        Self { children: [NONE; 26], count: 0 }
    }
}

fn count_prefix_suffix_pairs(words: Vec<String>) -> i64 {
    let m = words.iter().max_by_key(|v| v.len()).unwrap().len();
    let mut pow = vec![0; m];
    pow[0] = 1;
    for i in 1..m {
        pow[i] = pow[i-1] * BASE % MOD;
    }

    let mut trie = Box::new(Trie::new());
    for word in &words {
        let mut trie = trie.as_mut();
        let word = word.as_bytes();
        let n = word.len();
        let mut hashed_left = 0;
        let mut hashed_right = 0;
        for i in 0..n {
            hashed_left = (hashed_left * BASE % MOD + (word[i] - b'a') as i64) % MOD;
            hashed_right = ((word[n-i-1] - b'a') as i64 * pow[i] % MOD + hashed_right) % MOD;
            trie = trie.children[(word[i] - b'a') as usize].get_or_insert_with(|| Box::new(Trie::new()));
        }

        trie.count += 1;
    }

    let mut count = 0;
    for word in words.into_iter().rev() {
        let mut trie = trie.as_mut();
        let word = word.as_bytes();
        let n = word.len();
        let mut hashed_left = 0;
        let mut hashed_right = 0;
        for i in 0..n {
            hashed_left = (hashed_left * BASE % MOD + (word[i] - b'a') as i64) % MOD;
            hashed_right = ((word[n-i-1] - b'a') as i64 * pow[i] % MOD + hashed_right) % MOD;
            trie = trie.children[(word[i] - b'a') as usize].as_mut().unwrap();
            if hashed_left == hashed_right {
                if i == n-1 { trie.count -= 1; }
                count += trie.count;
            }
        }
    }

    count
}

pub fn main() {
    let words = ["a","aba","ababa","aa"].into_iter().map(String::from).collect();
    println!("{}", count_prefix_suffix_pairs(words));
}
