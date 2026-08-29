pub fn make_the_integer_zero(num1: i32, num2: i32) -> i32 {
    let(mut num1, num2) = (num1 as i64, num2 as i64);
    let mut res = 0;

    loop {
        res += 1;
        num1 -= num2;
        if num1 < res {return -1;}
        if (num1.count_ones() as i64) <= res {
            return res as _;
        }
    }
}

pub fn main() {
    let num1 = 5;
    let num2 = 1;
    println!("{}", make_the_integer_zero(num1, num2));
}
