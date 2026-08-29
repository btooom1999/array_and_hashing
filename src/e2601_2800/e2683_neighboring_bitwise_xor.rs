fn does_valid_array_exist(derived: Vec<i32>) -> bool {
    let first = 0;
    let mut current = 0;
    for (i, &num) in derived.iter().enumerate() {
        if i == derived.len()-1 {
            if current ^ first == num {
                return true;
            }
        } else {
            current ^= num;
        }
    }

    false
}

pub fn main() {
    let derived = [1,1].to_vec();
    println!("{}", does_valid_array_exist(derived));
}
