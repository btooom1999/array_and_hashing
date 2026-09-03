#[derive(Debug)]
struct Trie {
    children: [Option<Box<Trie>>; 26],
    at: usize,
}

impl Trie {
    fn new(at: usize) -> Self {
        const NONE: Option<Box<Trie>> = None;
        Self { children: [NONE; 26], at }
    }
}

fn string_indices(words_container: Vec<String>, words_query: Vec<String>) -> Vec<i32> {
    let mut trie = Box::new(Trie::new(0));
    let n = words_container.iter().len();
    let mut min = 0;
    for i in 0..n {
        if words_container[min].len() > words_container[i].len() {
            min = i;
        }
        let mut trie = trie.as_mut();
        for c in words_container[i].chars().rev() {
            trie = trie.children[(c as u8 - b'a') as usize].get_or_insert_with(|| Box::new(Trie::new(i)));
            if words_container[trie.at].len() > words_container[i].len() {
                trie.at = i;
            }
        }
    }

    trie.at = min;

    let mut res = vec![];
    for w in words_query {
        let mut trie = trie.as_mut();
        for c in w.chars().rev() {
            if let Some(next) = trie.children[(c as u8 - b'a') as usize].as_mut() {
                trie = next;
            } else {
                break;
            }
        }

        res.push(trie.at as i32);
    }

    res
}

pub fn main() {
    let words_container = ["abcd","bcd","xbcd"].into_iter().map(String::from).collect();
    let words_query = ["cd","bcd","xyz"].into_iter().map(String::from).collect();
    println!("{:?}", string_indices(words_container, words_query));
}
