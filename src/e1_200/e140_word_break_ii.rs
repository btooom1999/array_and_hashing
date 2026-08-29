struct Trie {
    children: [Option<Box<Trie>>; 26],
    end: bool,
}

impl Trie {
    fn new() -> Self {
        const NONE: Option<Box<Trie>> = None;
        Self { children: [NONE; 26], end: false }
    }
}

fn backtracking(
    current_trie: &Trie,
    trie: &Trie,
    s: &[u8],
    i: usize,
    str: &mut String,
    result: &mut Vec<String>,
) {
    if i == s.len() {
        if current_trie.end {
            result.push(str.trim().to_string());
        }
        return;
    }

    if let Some(current_trie) = current_trie.children[(s[i] - b'a') as usize].as_deref() {
        str.push(s[i] as char);
        if current_trie.end {
            str.push(' ');
            backtracking(trie, trie, s, i+1, str, result);
            str.pop();
        }

        backtracking(current_trie, trie, s, i+1, str, result);
        str.pop();
    }
}

fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
    let mut trie = Box::new(Trie::new());
    for word in word_dict {
        let mut trie = trie.as_mut();
        for b in word.as_bytes() {
            trie = trie.children[(b - b'a') as usize].get_or_insert_with(|| Box::new(Trie::new()));
        }

        trie.end = true;
    }

    let mut result = Vec::new();
    backtracking(&trie, &trie, s.as_bytes(), 0, &mut String::new(), &mut result);
    result
}

pub fn main() {
    let s = "catsanddog".to_string();
    let word_dict = ["cat","cats","and","sand","dog"].into_iter().map(String::from).collect();
    println!("{:?}", word_break(s, word_dict));
}
