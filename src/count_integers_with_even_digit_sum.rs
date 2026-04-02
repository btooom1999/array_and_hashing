fn count_even(num: i32) -> i32 {
    (1..=num).fold(0, |acc, mut num| {
        let mut sum = 0;
        while num > 0 {
            sum += num % 10;
            num /= 10;
        }
        acc + if sum % 2 == 0 { 1 } else { 0 }
    })
}

pub fn main() {
    let num = 30;
    println!("{}", count_even(num));
}
