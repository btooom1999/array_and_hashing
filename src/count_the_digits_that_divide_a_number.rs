fn count_digits(num: i32) -> i32 {
    let mut count = 0;
    let mut x = num;
    while x > 0 {
        if num % (x % 10) == 0 {
            count += 1;
        }
        x /= 10;
    }

    count
}

pub fn main() {
    let num = 7;
    println!("{}", count_digits(1));
}
