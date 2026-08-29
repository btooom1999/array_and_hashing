use std::collections::HashMap;

fn valid_utf8(data: Vec<i32>) -> bool {
    let hashmap = HashMap::from([
        (0b11000000, 1),
        (0b11100000, 2),
        (0b11110000, 3),
    ]);

    let mut need = 0;
    for num in data {
        let mut k = 0;
        for i in (3..8).rev() {
            if (1 << i) & num == 0 { break; }
            k |= (1 << i) & num;
        }

        if k == 0 || k == 0b11000000 || k == 0b11100000 || k == 0b11110000 {
            if need > 0 { return false; }
            need = *hashmap.get(&k).unwrap_or(&0);
        } else if k == 0b10000000 {
            if need == 0 { return false; }
            need -= 1;
        } else {
            return false;
        }
    }

    need == 0
}

pub fn main() {
    let data = [248,130,130,130].to_vec();
    println!("{}", valid_utf8(data));
}
