#[derive(Debug)]
struct TrieNode {
    children: [Option<Box<TrieNode>>; 2],
}

impl TrieNode {
    fn new() -> Self {
        const NONE: Option<Box<TrieNode>> = None;
        Self { children: [NONE; 2] }
    }
}

fn find_maximum_xor(nums: Vec<i32>) -> i32 {
    let mut root = TrieNode::new();

    for &num in &nums {
        let mut trie = &mut root;
        for i in (0..32).rev() {
            let k = ((1 << i & num) > 0) as usize;
            trie = trie.children[k].get_or_insert_with(|| Box::new(TrieNode::new()));
        }
    }

    let mut res = 0;
    for mut num in nums {
        let mut trie = &mut root;
        for i in (0..32).rev() {
            if num < res {
                continue;
            }

            let k = (1 << i & num) == 0;
            if trie.children[k as usize].is_some() {
                if k { num ^= 1 << i; }
                trie = trie.children[k as usize].as_mut().unwrap();
            } else if trie.children[!k as usize].is_some() {
                if !k { num ^= 1 << i };
                trie = trie.children[!k as usize].as_mut().unwrap();
            }
        }

        res = res.max(num);
    }

    res
}

fn generate_nums() -> Vec<i32> {
    (0..=200_000).map(|_| rand::random_range(0..=i32::MAX)).collect()
}

pub fn main() {
    // let nums = [3].to_vec();
    let nums = [4,5,6].to_vec();
    // let nums = generate_nums();
    println!("{}", find_maximum_xor(nums));
}
