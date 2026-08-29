struct TrieNode {
    hashmap: [Option<Box<TrieNode>>; 26],
    count: i32,
}

impl TrieNode {
    fn new() -> Self {
        const NONE: Option<Box<TrieNode>> = None;
        Self { hashmap: [NONE; 26], count: 0 }
    }
}

fn sum_prefix_scores(words: Vec<String>) -> Vec<i32> {
    let mut root = TrieNode::new();

    for w in &words {
        let mut cur_root = &mut root;
        for b in w.as_bytes() {
            let i = (b - b'a') as usize;
            cur_root = cur_root.hashmap[i].get_or_insert_with(|| Box::new(TrieNode::new()));
            cur_root.count += 1;
        }
    }

    words.iter().map(|w| {
        let mut cur_root = &mut root;
        let mut sum = 0;
        for b in w.as_bytes() {
            let i = (b - b'a') as usize;
            cur_root = cur_root.hashmap[i].get_or_insert_with(|| Box::new(TrieNode::new()));
            sum += cur_root.count;
        }

        sum
    }).collect()
}

pub fn main() {
    let words = ["abc", "ab", "bc", "b"].into_iter().map(String::from).collect::<Vec<_>>();
    println!("{:?}", sum_prefix_scores(words));
}
