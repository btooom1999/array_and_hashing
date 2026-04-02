fn add_digits(num: i32) -> i32 {
    1 + (num-1) % 9
}

pub fn main() {
    let num = i32::MAX;
    println!("{}", add_digits(num));
}
