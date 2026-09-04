use std::collections::HashMap;

#[derive(Debug, Default)]
struct Node {
    hashmap: HashMap<char, usize>,
    fail: usize,
    indexes: Vec<usize>,
}

#[derive(Debug)]
struct AhoCorasick {
    nodes: Vec<Node>,
}

impl AhoCorasick {
    fn new(words: Vec<String>) -> Self {
        let mut aho_corasick = Self { nodes: vec![Node::default()] };
        for (i, w) in words.into_iter().enumerate() {
            aho_corasick.build_trie(w, i);
        }

        aho_corasick.build_failure_links();
        aho_corasick
    }

    fn build_trie(&mut self, w: String, i: usize) {
        let mut cur = 0;
        for c in w.chars() {
            let n = self.nodes.len();
            if let std::collections::hash_map::Entry::Vacant(e) = self.nodes[cur].hashmap.entry(c) {
                e.insert(n);
                self.nodes.push(Node::default());
            }

            cur = *self.nodes[cur].hashmap.get(&c).unwrap();
        }

        self.nodes[cur].indexes.push(i);
    }

    fn build_failure_links(&mut self) {
        let mut q = std::collections::VecDeque::new();
        for &i in self.nodes[0].hashmap.values() {
            q.push_back(i);
        }

        while let Some(u) = q.pop_front() {
            let u_next = self.nodes[u].hashmap.clone();
            for (c, &v) in &u_next {
                let mut f = self.nodes[u].fail;
                while f > 0 && !self.nodes[f].hashmap.contains_key(c) {
                    f = self.nodes[f].fail;
                }

                if let Some(&next) = self.nodes[f].hashmap.get(c) && next != v {
                    self.nodes[v].fail = next;
                }

                let fail = self.nodes[v].fail;
                let addtional_indexes = self.nodes[fail].indexes.clone();
                self.nodes[v].indexes.extend(addtional_indexes);

                q.push_back(v);
            }
        }
    }
}

fn minimum_cost(target: String, words: Vec<String>, costs: Vec<i32>) -> i32 {
    let aho_corasick = AhoCorasick::new(words.clone());

    let n = target.len();
    let mut dp = vec![i32::MAX; n];
    let mut cur = 0;
    for (j, c) in target.chars().enumerate() {
        while cur > 0 && !aho_corasick.nodes[cur].hashmap.contains_key(&c) {
            cur = aho_corasick.nodes[cur].fail;
        }

        if let Some(&k) = aho_corasick.nodes[cur].hashmap.get(&c) {
            cur = k;
            for &k in &aho_corasick.nodes[cur].indexes {
                let i = j+1-words[k].len();
                let amount = if i == 0 { 0 } else { dp[i-1] };
                if amount != i32::MAX {
                    dp[j] = dp[j].min(costs[k] + amount);
                }
            }
        } else {
            break;
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
