fn bitwise_complement(num: i32) -> i32 {
    if num == 0 { return 1; }

    let bits = 32 - num.leading_zeros() as i32;
    num ^ ((1 << bits) - 1)
}

pub fn main() {
    let num = 5;
    println!("{}", bitwise_complement(num));
}
