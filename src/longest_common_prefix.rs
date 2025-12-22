fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.len() == 1 {
        return strs[0].clone();
    }

    let mut min = strs[0].len();
    for str in &strs {
        min = std::cmp::min(min, str.len());
    }

    let mut result = Vec::with_capacity(min);

    for i in 0..min {
        let num = strs[0].as_bytes()[i];
        let mut flag = true;

        for str in &strs {
            if str.as_bytes()[i] != num {
                flag = false;
                break;
            }
        }

        if !flag {
            break;
        }

        result.push(num);
    }

    String::from_utf8(result).unwrap()
}

pub fn main() {
    let strs = ["", ""]
        .into_iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>();

    println!("{}", longest_common_prefix(strs));
}
