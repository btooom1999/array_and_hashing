fn is_power_of_three(n: i32) -> bool {
    n > 0 && 3i32.pow(19) % n == 0
}

pub fn main() {
    let n = 27;
    println!("{}", is_power_of_three(n));
}
