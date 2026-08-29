fn common_factors(a: i32, b: i32) -> i32 {
    let mut count = 0;
    for i in 1..=1000 {
        if i > a || i > b {
            break;
        }

        if a % i == 0 && b % i == 0 {
            count += 1;
        }
    }

    count
}

pub fn main() {
    let a = 12;
    let b = 6;
    println!("{}", common_factors(a, b));
}
