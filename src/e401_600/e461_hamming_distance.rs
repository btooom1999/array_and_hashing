fn hamming_distance(x: i32, y: i32) -> i32 {
    (x^y).count_ones() as i32
}

pub fn main() {
    let x = 1;
    let y = 1;
    println!("{}", hamming_distance(x, y));
}
