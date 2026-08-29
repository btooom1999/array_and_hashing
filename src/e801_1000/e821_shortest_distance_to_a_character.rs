fn shortest_to_char(s: String, c: char) -> Vec<i32> {
    let mut indexes = Vec::new();
    for (i, l) in s.chars().enumerate() {
        if c == l {
            indexes.push(i);
        }
    }

    let mut res = Vec::new();
    for i in 0..s.len() {
        let mut min = i32::MAX;
        for &j in &indexes {
            min = min.min((j as i32 - i as i32).abs());
        }
        res.push(min);
    }

    res
}

pub fn main() {
    let s = "loveleetcode".to_string();
    let c = 'e';
    println!("{:?}", shortest_to_char(s, c));
}
