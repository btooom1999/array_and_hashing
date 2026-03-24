#[derive(Debug, Default, Clone)]
struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
}

impl TrieNode {
    fn new() -> Self {
        const NONE: Option<Box<TrieNode>> = None;
        Self { children: [NONE; 26] }
    }
}

fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
    let mut root = TrieNode::new();

    fn dfs(
        root: Box<TrieNode>,
        node: Box<TrieNode>,
        start: bool,
        idx: usize,
        s: &[u8],
    ) -> i32 {
        if idx >= s.len() {
            return 0;
        }

        let k = (s[idx] - b'a') as usize;

        if node.children[k].is_some() {
            return dfs(root, node.children[k].clone().unwrap(), false, idx+1, s);
        }

        if start {
            return 1 + dfs(root.clone(), root.clone(), true, idx+1, s);
        }

        dfs(root.clone(), root.clone(), true, idx, s)
    }

    for dict in dictionary {
        let mut curr = &mut root;
        for b in dict.as_bytes() {
            let i = (b - b'a') as usize;
            curr = curr.children[i].get_or_insert_default();
        }
    }

    dfs(Box::new(root.clone()), Box::new(root.clone()), true, 0, s.as_bytes())
}

pub fn main() {
    let s = "leetscode".to_string();
    let dictionary = ["leet","code", "leetcode"].into_iter().map(String::from).collect::<Vec<_>>();
    // let s = "metzeaencgpgvsckjrqafkxgyzbe".to_string();
    // let dictionary = ["zdzz","lgrhy","r","ohk","zkowk","g","zqpn","anoni","ka","qafkx","t","jr","xdye","mppc","bqqb","encgp","yf","vl","ctsxk","gn","cujh","ce","rwrpq","tze","zxhg","yzbe","c","o","hnk","gv","uzbc","xn","kk","ujjd","vv","mxhmv","ugn","at","kumr","ensv","x","uy","gb","ae","jljuo","xqkgj"].into_iter().map(String::from).collect::<Vec<_>>();

    println!("{}", min_extra_char(s, dictionary));
}
