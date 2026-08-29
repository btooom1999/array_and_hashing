fn smallest_number(n: i32) -> i32 {
    let mut mask = 0;
    loop {
        if mask >= n {
            return mask;
        }

        mask = (mask << 1) | 1;
    }
}

pub fn main() {
    let n = 5;
    println!("{}", smallest_number(n));
}
