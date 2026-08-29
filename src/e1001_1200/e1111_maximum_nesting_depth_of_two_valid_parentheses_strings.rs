fn max_depth_after_split(seq: String) -> Vec<i32> {
    let mut depthest = 0;
    let mut depth = 0;
    for c in seq.chars() {
        if c == '(' {
            depth += 1;
            depthest = depthest.max(depth);
        } else {
            depth -= 1;
        }
    }

    let avg = depthest / 2;
    let mut res = vec![0; seq.len()];
    for (i, c) in seq.chars().enumerate() {
        if c == '(' {
            if depth >= avg {
                res[i] = 1;
            }
            depth += 1;
        } else {
            depth -= 1;
            if depth >= avg {
                res[i] = 1;
            }
        }
    }

    res
}

pub fn main() {
    let seq = "(()())".to_string();
    println!("{:?}", max_depth_after_split(seq));
}
