fn smallest_even_multiple(n: i32) -> i32 {
    if n % 2 == 0 { n } else { n * 2 }
}

pub fn main() {
    let n = 6;
    println!("{}", smallest_even_multiple(n));
}
