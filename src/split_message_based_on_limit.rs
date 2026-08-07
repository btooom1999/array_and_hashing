fn split_message(message: String, limit: i32) -> Vec<String> {
    let n = message.len();
    let limit = limit as usize;
    for amount in 1..5 {
        let mut res = Vec::new();
        let dots = std::iter::repeat_n('.', amount).collect::<String>();
        let mut at = 1;
        let mut pattern = format!("<{}/{}>", at, dots);
        let mut str = String::new();
        let mut flag = true;
        for (i, c) in message.chars().enumerate() {
            str.push(c);
            if (i == n-1 && str.len() + pattern.len() <= limit) || str.len() + pattern.len() == limit {
                res.push(format!("{}{}", str, pattern));
                str.clear();
                at += 1;
                pattern = format!("<{}/{}>", at, dots);
            } else if str.len() + pattern.len() > limit {
                flag = false;
                break;
            }
        }

        if flag && res.len().to_string().len() == amount {
            let value = res.len().to_string();
            return res.into_iter().map(|v| v.replace(&dots, &value)).collect();
        }
    }

    vec![]
}

pub fn main() {
    let message = "this is really a very awesome message".to_string();
    let limit = 9;
    // let message = "abbababbbaaa aabaa a".to_string();
    // let limit = 8;
    println!("{:?}", split_message(message, limit));
}
