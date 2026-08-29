fn find_complement(num: i32) -> i32 {
    let mut temp = num;
    let mut mask = 0;
    while temp > 0 {
        mask = mask << 1 | 1;
        temp >>= 1;
    }

    num ^ mask
}

pub fn main() {
    let num = 5;
    println!("{}", find_complement(num));
}
