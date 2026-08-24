use std::collections::{HashMap, HashSet};

const MOD: i64 = 2i64.pow(45)-1;
const BASE: i64 = 26;

fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
    let n = words.len();
    let mut hashmap = HashMap::new();
    for i in 0..n {
        let m = words[i].len();
        let word = words[i].as_bytes();
        let mut palindromic = true;
        let mut hashed = 0;
        for j in 0..m {
            if palindromic && word[j] != word[m-j-1] {
                palindromic = false;
            }
            hashed = (hashed * BASE % MOD + (word[j] + 1 - b'a') as i64) % MOD;
        }

        hashmap.insert(hashed, (i, palindromic));
    }

    let mut res = HashSet::new();
    for (&hashed, &(k, palindromic)) in &hashmap {
        if hashed == 0 { continue; }

        if palindromic && let Some(&(j, _)) = hashmap.get(&0) {
            res.insert(vec![k as i32, j as i32]);
            res.insert(vec![j as i32, k as i32]);
        }

        let mut word = vec![];
        word.push(b'$');
        for c in words[k].chars() {
            word.push(b'#');
            word.push(c as u8 - b'a');
        }
        word.push(b'#');
        word.push(b'%');

        let (mut c, mut r) = (0, 0);
        let m = word.len();
        let mut p = vec![0; m];
        for i in 1..m-1 {
            if r>i {
                let i_mirror = 2*c-i;
                p[i] = p[i_mirror].min(r-i);
            }

            while word[i-1-p[i]] == word[i+1+p[i]] {
                p[i] += 1;
            }

            if i+p[i] > r {
                c = i;
                r = i+p[i];
            }
        }

        let mut hashed = 0;
        for (j, c) in words[k].chars().rev().enumerate() {
            let skip = (j+1)*2;
            let idx = (m-2-skip)/2+1;
            hashed = (hashed * BASE % MOD + (c as u8 + 1 - b'a') as i64) % MOD;
            if (j+1 == words[k].len() || p[idx]*2+1 >= m-2-skip) && let Some(&(i, _)) = hashmap.get(&hashed) && k != i {
                res.insert(vec![i as i32, k as i32]);
            }
        }

        hashed = 0;
        let mut pow = 1;
        for (j, c) in words[k].chars().enumerate() {
            let skip = (j+1)*2;
            let idx = (m-2-skip)/2+1+skip;
            hashed = (hashed + (c as u8 + 1 - b'a') as i64 * pow % MOD) % MOD;
            pow = pow * BASE % MOD;
            if (j+1 == words[k].len() || p[idx]*2+1 >= m-2-skip) && let Some(&(j, _)) = hashmap.get(&hashed) && k != j {
                res.insert(vec![k as i32, j as i32]);
            }
        }
    }

    res.into_iter().collect()
}

pub fn main() {
    // let words = ["abcd","dcba","lls", "llls","s","sssll"].into_iter().map(String::from).collect();
    let words = ["a", "abc", "aba", ""].into_iter().map(String::from).collect();
    println!("{:?}", palindrome_pairs(words));
}
