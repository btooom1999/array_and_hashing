fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
    let mut word_list = word_list.into_iter().collect::<std::collections::HashSet<_>>();
    if !word_list.contains(&end_word) { return 0; }

    let mut queue = std::collections::VecDeque::from([(begin_word, 0)]);
    while let Some((mut begin_word, count)) = queue.pop_front() {
        if begin_word == end_word { return count+1; }

        for i in 0..begin_word.len() {
            let begin_word = unsafe { begin_word.as_bytes_mut() };
            let old = begin_word[i];
            for k in 0..26 {
                if (k + b'a') != old {
                    begin_word[i] = k + b'a';
                    let str = String::from_utf8(begin_word.to_vec()).unwrap();
                    if word_list.contains(&str) {
                        word_list.remove(&str);
                        queue.push_back((str, count+1));
                    }
                }
            }

            begin_word[i] = old;
        }
    }

    0
}

pub fn main() {
    let begin_word = "hit".to_string();
    let end_word = "cog".to_string();
    let word_list = ["hot","dot","dog","lot","log","cog"].into_iter().map(String::from).collect();
    println!("{}", ladder_length(begin_word, end_word, word_list));
}
