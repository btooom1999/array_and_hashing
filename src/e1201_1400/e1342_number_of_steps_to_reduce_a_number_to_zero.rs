fn number_of_steps(mut num: i32) -> i32 {
    let mut step = 0;
    while num > 0 {
        step += 1;
        if num % 2 == 0 {
            num >>= 1;
        } else {
            num -= 1;
        }
    }

    step
}

pub fn main() {
    let num = 14;
    println!("{}", number_of_steps(num));
}
