struct Trie {
    children: [Option<Box<Trie>>; 26],
    cost: i32,
}

impl Trie {
    fn new() -> Self {
        const NONE: Option<Box<Trie>> = None;
        Self { children: [NONE; 26], cost: i32::MAX }
    }
}

fn minimum_cost(target: String, words: Vec<String>, costs: Vec<i32>) -> i32 {
    let mut trie = Box::new(Trie::new());
    for (i, word) in words.iter().enumerate() {
        let mut trie = trie.as_mut();
        for b in word.bytes() {
            trie = trie.children[(b - b'a') as usize].get_or_insert_with(|| Box::new(Trie::new())).as_mut();
        }

        trie.cost = trie.cost.min(costs[i]);
    }

    let target = target.as_bytes();
    let n = target.len();
    let mut dp = vec![i32::MAX; n];
    for i in 0..n {
        let mut k = 0;
        let mut trie = trie.as_mut();
        while i+k < n {
            if let Some(next) = trie.children[(target[i+k] - b'a') as usize].as_mut() {
                trie = next;
            } else {
                break;
            }

            if trie.cost < i32::MAX {
                let prev = if i == 0 { 0 } else { dp[i-1] };
                dp[i+k] = dp[i+k].min(trie.cost.saturating_add(prev));
            }

            k += 1;
        }
    }

    if dp[n-1] == i32::MAX { -1 } else { dp[n-1] }
}

pub fn main() {
    let target = "abcdef".to_string();
    let words = ["abdef","abc","d","def","ef"].into_iter().map(String::from).collect();
    let costs = [100,1,1,10,5].to_vec();
    println!("{}", minimum_cost(target, words, costs));
}
