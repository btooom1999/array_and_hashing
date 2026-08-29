fn gray_code(n: i32) -> Vec<i32> {
    (1..2i32.pow(n as u32)).map(|i| i ^ (i/2)).collect::<Vec<_>>()
}

pub fn main() {
    let n = 3;
    println!("{:?}", gray_code(n));
}
