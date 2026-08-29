use std::collections::HashMap;

fn confusing_number(n: i32) -> bool {
    let hashmap = HashMap::<i32, i32>::from([(0, 0), (1, 1), (6, 9), (8, 8), (9, 6)]);

    let mut rotated_n = 0;
    let mut temp_n = n;
    while temp_n > 0 {
        let val = temp_n % 10;
        if let Some(val) = hashmap.get(&val) {
            rotated_n = rotated_n * 10 + val;
        } else {
            return false;
        }

        temp_n /= 10;
    }

    rotated_n != n
}

pub fn main() {
    let n = 89;
    println!("{}", confusing_number(n));
}
