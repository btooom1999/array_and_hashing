fn check_perfect_number(num: i32) -> bool {
    if num <= 1 {
        return false;
    }

    let mut sum = 0;
    for i in 1..=num.isqrt() {
        if num % i == 0 {
            sum += i;

            if i != 1 {
                sum += num / i;
            }
        }
    }

    sum == num
}

pub fn main() {
    let num = 28;
    println!("{}", check_perfect_number(num));
}
