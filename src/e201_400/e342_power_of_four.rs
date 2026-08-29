fn is_power_of_four(n: i32) -> bool {
    // 1431655765 is both 0x55555555 and 01010101010101010101010101010101 (ones are odd indexes)
    n > 0 && n & (n-1) == 0 && n & 1431655765 != 0
}

pub fn main() {
    let n = 8;
    println!("{}", is_power_of_four(n));
}
