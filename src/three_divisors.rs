fn is_three(n: i32) -> bool {
    (2..=n/2).filter(|x| n % x == 0).count() == 1
}

pub fn main() {
    let n = 14;
    println!("{}", is_three(n));
}
