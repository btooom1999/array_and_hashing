#[derive(Debug)]
struct TrieNode {
    count: i32,
    children: [Option<Box<TrieNode>>; 26],
}

impl TrieNode {
    fn new() -> Self {
        const NONE: Option<Box<TrieNode>> = None;
        Self { children: [NONE; 26], count: 0 }
    }
}

fn dfs(
    allow: bool,
    s: &[u8],
    i: usize,
    trie: &TrieNode,
) -> i32 {
    if i == s.len() {
        return if allow { 0 } else { trie.count }
    }

    let mut res = 0;
    for x in 0..26 {
        if trie.children[x].is_some() {
            if (s[i]-b'a') as usize == x  {
                res += dfs(allow, s, i+1, trie.children[x].as_ref().unwrap());
            } else if allow {
                res += dfs(false, s, i+1, trie.children[x].as_ref().unwrap());
            }
        }
    }

    res
}

fn count_substrings(s: String, t: String) -> i32 {
    let s = s.as_bytes();
    let t = t.as_bytes();
    let (sn, tn) = (s.len(), t.len());
    let mut root = TrieNode::new();
    for i in 0..tn {
        let mut trie = &mut root;
        for j in i..tn {
            trie = trie.children[(t[j]-b'a') as usize].get_or_insert_with(|| Box::new(TrieNode::new()));
            trie.count += 1;
        }
    }

    let mut res = 0;
    for i in 0..sn {
        for j in i..sn {
            res += dfs(true, &s[i..=j], 0, &root);
        }
    }

    res
}

pub fn main() {
    let s = "aba".to_string();
    let t = "baba".to_string();
    println!("{}", count_substrings(s, t));
}
