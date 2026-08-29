fn add_spaces(s: String, spaces: Vec<i32>) -> String {
    let s = s.as_bytes();
    let mut res = String::new();

    let mut l = 0;
    let mut i = 0;
    while l < s.len() {
        if let Some(idx) = spaces.get(i) && l == *idx as usize {
            res.push(' ');
            i += 1;
        }
        res.push(s[l] as char);
        l += 1;
    }

    res
}

pub fn main() {
    let s = "LeetcodeHelpsMeLearn".to_string();
    let spaces = vec![8,13,15];
    println!("{}", add_spaces(s, spaces));
}
