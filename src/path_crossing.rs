use std::collections::HashSet;

fn is_path_crossing(path: String) -> bool {
    let mut hashset = HashSet::<(i32, i32)>::with_capacity(path.len());
    hashset.insert((0, 0));

    let mut origin = (0, 0);
    for d in path.chars() {
        if d == 'N' {
            origin = (origin.0, origin.1 + 1);
        } else if d == 'E' {
            origin = (origin.0 + 1, origin.1);
        } else if d == 'S' {
            origin = (origin.0, origin.1 - 1);
        } else {
            origin = (origin.0 - 1, origin.1)
        }

        if !hashset.contains(&origin) {
            hashset.insert(origin);
        } else {
            return true;
        }
    }

    false
}

pub fn main() {
    let path = "NES".to_string();
    println!("{}", is_path_crossing(path));
}
