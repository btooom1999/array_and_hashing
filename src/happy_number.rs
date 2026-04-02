use std::collections::HashMap;

fn is_happy(n: i32) -> bool {
    let mut hashmap = HashMap::new();
    hashmap.insert(1, true);

    let mut x = n;
    loop {
        let mut sum = 0;
        if let Some(&x) = hashmap.get(&x) {
            return x;
        }

        hashmap.insert(x, false);

        while x > 0 {
            let num = x%10;
            sum += num * num;
            x /= 10;
        }

        x = sum;
    }
}

pub fn main() {
    let n = i32::MAX;
    println!("{}", is_happy(n));
}
