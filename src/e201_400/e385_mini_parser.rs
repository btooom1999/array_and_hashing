
#[derive(Debug)]
enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

fn iterator(mut s: &str) -> NestedInteger {
    if s == "[]" {
        return NestedInteger::List(vec![]);
    }
    if let Ok(num) = s.parse::<i32>() {
        return NestedInteger::Int(num);
    }

    if s.starts_with('[') {
        s = &s[1..s.len()-1];
    }

    let mut list = vec![];
    let mut i = 0;
    let mut brackets = 0;
    #[allow(clippy::char_indices_as_byte_indices)]
    for (j, c) in s.chars().enumerate() {
        if c == ',' && brackets == 0 {
            list.push(iterator(&s[i..j]));
            i = j+1;
        } else if c == '[' {
            brackets += 1;
        } else if c == ']' {
            brackets -= 1;
        }
    }

    list.push(iterator(&s[i..]));

    NestedInteger::List(list)
}

fn deserialize(s: String) -> NestedInteger {
    iterator(&s)
}

pub fn main() {
    let s = "[123,[456,[789]]]".to_string();
    println!("{:?}", deserialize(s));
}
