#[derive(Debug)]
struct TrieNode {
    is_end: bool,
    children: [Option<Box<TrieNode>>; 75],
}

impl TrieNode {
    fn new() -> Self {
        const NONE: Option<Box<TrieNode>> = None;
        Self { is_end: false, children: [NONE; 75] }
    }
}

fn add_bold_tag(s: String, words: Vec<String>) -> String {
    let mut trie = Box::new(TrieNode::new());

    for word in words {
        let mut trie = &mut trie;
        for b in word.as_bytes() {
            trie = trie.children[(b-b'0') as usize].get_or_insert_with(|| Box::new(TrieNode::new()));
        }
        trie.is_end = true;
    }

    let s = s.as_bytes();
    let n = s.len();
    let mut ranges = Vec::<(usize, usize)>::new();
    for i in 0..n {
        let mut trie = Some(&mut trie);
        let mut j = i;
        let mut distance = usize::MAX;
        while j < n {
            let temp = trie.unwrap();
            trie = temp.children[(s[j]-b'0') as usize].as_mut();
            if let Some(trie) = &trie {
                if trie.is_end {
                    distance = j;
                }
            } else {
                break;
            }

            j += 1;
        }

        if distance != usize::MAX {
            if let Some(last) = ranges.last_mut() && last.1+1 >= i {
                last.1 = distance.max(last.1);
            } else {
                ranges.push((i, distance));
            }
        }
    }

    let mut i = 0;
    let mut res = Vec::new();
    for j in 0..s.len() {
        if let Some(range) = ranges.get(i) && j == range.0 {
            res.extend("<b>".as_bytes());
        }
        res.push(s[j]);
        if let Some(range) = ranges.get(i) && j == range.1 {
            res.extend("</b>".as_bytes());
            i += 1;
        }
    }

    String::from_utf8(res).unwrap()
}

fn generate_worst_case() -> (String, Vec<String>) {
    let s = "a".repeat(1000);
    let mut words = Vec::new();
    for i in 1..=100 {
        words.push("a".repeat(i));
    }

    (s, words)
}

pub fn main() {
    // let s = "aaabbbbb".to_string();
    // let words = vec!["abc", "123"].into_iter().map(String::from).collect();

    let (s, words) = generate_worst_case();
    println!("{}", add_bold_tag(s, words));
}
