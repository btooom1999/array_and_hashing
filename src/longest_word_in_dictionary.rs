use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug)]
struct TrieNode {
    hashmap: RefCell<HashMap<char, Rc<TrieNode>>>,
}

impl TrieNode {
    fn new() -> Self {
        Self { hashmap: RefCell::new(HashMap::default()) }
    }
}

fn longest_word(mut words: Vec<String>) -> String {
    words.sort();

    let mut trie = Rc::new(TrieNode::new());
    let mut max = (0, String::new());

    for w in words.into_iter() {
        let mut cur_trie = trie.clone();
        let mut i = 0;
        let w = w.as_bytes();

        let mut str = String::new();
        let mut cur_max = 0;

        while i < w.len() {
            let next_trie = {
                let borrow = cur_trie.hashmap.borrow();
                borrow.get(&(w[i] as char)).map(Rc::clone)
            };

            cur_max += 1;
            str.push(w[i] as char);

            if let Some(next) = next_trie {
                cur_trie = next.clone();
            } else if i + 1 == w.len() {
                if cur_max > max.0 || (cur_max == max.0 && str < max.1) {
                    max = (cur_max, str.clone());
                }
                cur_trie.hashmap.borrow_mut().insert(w[i] as char, Rc::new(TrieNode::new()));
            } else {
                break;
            }

            i += 1;
        }
    }

    max.1
}

pub fn main() {
    let words = ["a","banana","app","appl","ap","apply","apple"].into_iter().map(String::from).collect::<Vec<_>>();
    println!("{}", longest_word(words));
}
