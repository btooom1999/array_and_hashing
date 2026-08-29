fn alternate_digit_sum(mut n: i32) -> i32 {
    let mut multiply = if n.to_string().len() % 2 == 0 { -1 } else { 1 };
    let mut sum = 0;

    while n > 0 {
        sum += (n%10)*multiply;
        n /= 10;
        multiply *= -1;
    }

    sum
}

pub fn main() {
    let n = 886996;
    println!("{}", alternate_digit_sum(n));
}
