fn is_power_of_two(n: i32) -> bool {
    n > 0 && (n & (n-1)) == 0
}

pub fn main() {
    let n = 5;
    println!("{}", is_power_of_two(n));
}
