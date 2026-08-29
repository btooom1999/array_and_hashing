use std::collections::HashSet;

fn two_edit_words(queries: Vec<String>, dictionary: Vec<String>) -> Vec<String> {
    let mut hashset = HashSet::new();
    let mut res = Vec::new();
    for query in &queries {
        if hashset.contains(query) {
            res.push(query.clone());
            continue;
        }

        let bytes = query.clone().into_bytes();
        for dict in &dictionary {
            let dict = dict.as_bytes();
            let mut i = 0;
            let mut j = 0;
            let mut allowed_errors = 2;
            let mut flag = false;

            loop {
                if allowed_errors < 0 {
                    break;
                }

                match (bytes.get(i), dict.get(j)) {
                    (Some(val1), Some(val2)) => {
                        i += 1;
                        j += 1;

                        if val1 != val2 {
                            allowed_errors -= 1;
                        }
                    }
                    _ => {
                        hashset.insert(query.clone());
                        res.push(query.clone());
                        flag = true;
                        break;
                    }
                }
            }

            if flag {
                break;
            }
        }
    }

    res
}

pub fn main() {

}
