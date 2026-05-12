fn camel_match(queries: Vec<String>, pattern: String) -> Vec<bool> {
    let pattern = pattern.as_bytes();

    let mut res = Vec::new();
    for query in queries {
        let query = query.as_bytes();
        let mut i = 0;
        let mut j = 0;

        loop {
            match (query.get(i), pattern.get(j)) {
                (Some(val1), Some(val2)) => {
                    if val1.is_ascii_uppercase() && val2.is_ascii_uppercase() {
                        if val1 != val2 {
                            res.push(false);
                            break;
                        } else {
                            i += 1;
                            j += 1;
                        }
                    } else if val1.is_ascii_uppercase() {
                        res.push(false);
                        break;
                    } else if val2.is_ascii_uppercase() {
                        i += 1;
                    } else {
                        i += 1;
                        if val1 == val2 {
                            j += 1;
                        }
                    }
                }
                (_, Some(_)) => {
                    res.push(false);
                    break;
                }
                (Some(val), _) => {
                    if val.is_ascii_uppercase() {
                        res.push(false);
                        break;
                    }
                    i += 1;
                }
                (_, _) => {
                    res.push(true);
                    break;
                }
            }
        }
    }

    res
}

pub fn main() {
    let queries = ["FooBar","FooBarTest","FootBall","FrameBuffer","ForceFeedBack"].into_iter().map(String::from).collect::<Vec<_>>();
    let pattern = "FB".to_string();
    println!("{:?}", camel_match(queries, pattern));
}
