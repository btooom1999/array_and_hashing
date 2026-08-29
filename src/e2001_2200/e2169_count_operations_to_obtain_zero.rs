fn count_operations(mut num1: i32, mut num2: i32) -> i32 {
    let mut res = 0;
    while num1 != 0 && num2 != 0 {
        if num1 >= num2 {
            res += num1/num2;
            num1 %= num2;
        } else {
            res += num2/num1;
            num2 %= num1;
        }
    }

    res
}

pub fn main() {
    let num1 = 5;
    let num2 = 4;
    println!("{}", count_operations(num1, num2));
}
