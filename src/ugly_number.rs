fn is_ugly(mut n: i32) -> bool {
    if n < 0 {
        return false;
    }

    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
        } else if n % 3 == 0 {
            n /= 3;
        } else if n % 5 == 0 {
            n /= 5
        } else {
            return false;
        }
    }

    true
}

pub fn main() {
    let n = 14;
    println!("{}", is_ugly(n));
}
