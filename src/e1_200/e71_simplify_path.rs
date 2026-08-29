fn simplify_path(path: String) -> String {
    let path = path.as_bytes();
    let mut i = 0;
    let mut stack = Vec::new();
    while let Some(byte) = path.get(i) {
        if *byte == b'/' {
            while path.get(i).is_some_and(|v| *v == b'/') {
                i += 1;
            }
            if let Some(last_byte) = stack.last() {
                if *last_byte != b'/' {
                    stack.push(b'/');
                }
            } else {
                stack.push(b'/');
            }
        } else if *byte == b'.' {
            let mut end = i;
            while path.get(end).is_some_and(|v| *v == b'.') {
                end += 1;
            }

            if end - i == 2 {
                if stack.last().is_some_and(|v| *v == b'/') && path.get(end).is_none_or(|v| *v == b'/') {
                    if stack.len() > 1 {
                        stack.pop(); // remove last '/' character
                    }
                    while stack.last().is_some_and(|v| *v != b'/') {
                        stack.pop();
                    }
                } else {
                    stack.push(b'.');
                    stack.push(b'.');
                }
            } else if end - i > 2 {
                let directory = vec![b'.'; end - i];
                stack.extend(directory);
            } else if end - i == 1 && (stack.last().is_none_or(|v| *v != b'/') || path.get(end).is_some_and(|v| *v != b'/')) {
                stack.push(b'.');
            }

            i = end;
        } else {
            stack.push(*byte);
            i += 1;
        }
    }

    while stack.len() > 1 && stack.last().is_some_and(|v| *v == b'/') {
        stack.pop();
    }

    String::from_utf8(stack).unwrap()
}

pub fn main() {
    let path = "/.".to_string();
    println!("{}", simplify_path(path));
}
