fn min_operations(logs: Vec<String>) -> i32 {
    let mut level = 0;
    for log in logs {
        if log == "./" {
            continue;
        } else if log == "../" {
            level  = std::cmp::max(0, level - 1);
        } else {
            level += 1;
        }
    }

    level
}

pub fn main() {
    let logs = ["d1/","d2/","./","d3/","../","d31/"].into_iter().map(String::from).collect::<Vec<_>>();
    println!("{}", min_operations(logs));
}
