fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
    let mut i = 0;
    let mut count = 0;
    let mut len = 0;
    let mut res = vec![];
    for (j, word) in words.iter().enumerate() {
        if len + word.len() as i32 + count > max_width {
            let space = max_width - len;
            let mut str = String::new();
            if count > 1 {
                let mut extra = space % (count-1);
                let amount = space / (count-1);
                for i in i..j {
                    str.push_str(&words[i]);
                    if i+1 < j {
                        str.push_str(&std::iter::repeat_n(" ".to_string(), amount as usize).collect::<String>());
                        if extra > 0 {
                            str.push(' ');
                            extra -= 1;
                        }
                    }
                }
            } else {
                str.push_str(&words[i]);
                str.push_str(&std::iter::repeat_n(" ".to_string(), max_width as usize - str.len()).collect::<String>());
            }

            i = j;
            len = 0;
            count = 0;
            res.push(str);
        }

        count += 1;
        len += word.len() as i32;
    }

    let mut last = String::new();
    for i in i..words.len() {
        last.push_str(&words[i]);
        last.push(' ');
    }

    last.pop();
    last.push_str(&std::iter::repeat_n(" ".to_string(), max_width as usize - last.len()).collect::<String>());
    res.push(last);

    res
}

pub fn main() {
    let words = ["ask","not","what","your","country","can","do","for","you","ask","what","you","can","do","for","your","country"].into_iter().map(String::from).collect();
    let max_width = 16;
    println!("{:?}", full_justify(words, max_width));
}
