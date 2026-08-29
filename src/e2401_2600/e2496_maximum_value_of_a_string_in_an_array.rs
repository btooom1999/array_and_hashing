fn maximum_value(strs: Vec<String>) -> i32 {
    let mut res = 0;
    for str in strs {
        if let Ok(num) = str.parse::<i32>() {
            res = res.max(num);
        } else {
            res = res.max(str.len() as i32);
        }
    }

    res
}

pub fn main() {
    let strs = ["alic3","bob","3","4","00000"].into_iter().map(String::from).collect();
    println!("{}", maximum_value(strs));
}
