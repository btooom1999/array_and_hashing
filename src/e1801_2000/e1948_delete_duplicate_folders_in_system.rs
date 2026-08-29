use std::collections::{BTreeMap, HashMap, HashSet};

const MOD: i64 = 2i64.pow(45)-1;
const BASE: i64 = 26;

#[derive(Debug)]
struct Trie {
    children: Option<BTreeMap<String, Box<Trie>>>,
    index: usize,
}

impl Trie {
    fn new() -> Self {
        Self { children: None, index: 1_000_000_000 }
    }
}

fn dfs(
    trie: &Trie,
    hashmap: &mut HashMap<i64, Vec<usize>>,
    name: &str,
) -> String {
    if let Some(children) = trie.children.as_ref() {
        let mut str = String::new();
        for (name, next) in children {
            let next_name= dfs(next.as_ref(), hashmap, name);
            str.push_str(&format!("{{{next_name}}}"));
        }

        let mut hash = 0;
        for b in str.as_bytes() {
            hash = hash * BASE % MOD + (b - b'a' + 1) as i64;
        }

        hashmap.entry(hash).or_default().push(trie.index);
        format!("{{{}{}}}", name, str)
    } else {
        name.to_string()
    }
}

fn iterate(
    trie: &Trie,
    hashset: &mut HashSet<usize>,
    eliminate: bool,
) {
    let eliminate = eliminate || hashset.contains(&trie.index);
    if eliminate {
        hashset.insert(trie.index);
    }

    if let Some(children) = trie.children.as_ref() {
        for next in children.values() {
            iterate(next.as_ref(), hashset, eliminate);
        }
    }
}

fn delete_duplicate_folder(paths: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let n = paths.len();
    let mut trie = Trie::new();
    for i in 0..n {
        let mut trie = &mut trie;
        for path in &paths[i] {
            if trie.children.is_none() {
                trie.children = Some(BTreeMap::from([(path.clone(), Box::new(Trie::new()))]));
            } else if !trie.children.as_ref().unwrap().contains_key(path) {
                trie.children.as_mut().unwrap().insert(path.clone(), Box::new(Trie::new()));
            }

            trie = trie.children.as_mut().unwrap().get_mut(path).unwrap();
        }

        trie.index = i;
    }

    let mut hashmap = HashMap::new();
    dfs(&trie, &mut hashmap, "");

    let mut hashset = HashSet::<usize>::new();
    for indexes in hashmap.values() {
        if indexes.len() > 1 {
            for &i in indexes {
                hashset.insert(i);
            }
        }
    }

    iterate(&trie, &mut hashset, false);

    (0..n)
        .filter_map(|i| {
            if hashset.contains(&i) {
                None
            } else {
                Some(paths[i].clone())
            }
        })
        .collect()
}

pub fn main() {
    // let paths = vec![vec!["a"], vec!["a","x"], vec!["a","x","y"], vec!["a","z"], vec!["b"], vec!["b","x"], vec!["b","x","y"], vec!["b","w"], vec!["b","z"]]
    let paths = vec![vec!["a"],vec!["a","c"],vec!["a","d"],vec!["a","d","e"],vec!["b"],vec!["b","e"],vec!["b","c"],vec!["b","c","d"],vec!["f"],vec!["f","h"],vec!["f","h","i"],vec!["f","j"],vec!["g"],vec!["g","j"],vec!["g","h"],vec!["g","h","i"]]
        .into_iter()
        .map(|v| v.into_iter().map(String::from).collect())
        .collect();
    println!("{:?}", delete_duplicate_folder(paths));
}
