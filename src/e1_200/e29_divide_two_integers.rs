fn divide(a: i32, b: i32) -> i32 {
    if a == i32::MIN && b == -1 {
        return i32::MAX;
    }

    let negative = (a < 0) ^ (b < 0);
    let mut dividend = if a > 0 { -a } else { a };
    let divisor = if b > 0 { -b } else { b };
    let mut result = 0;

    while dividend <= divisor {
        let mut value = divisor;
        let mut multiple = 1;

        while value >= i32::MIN >> 1 && value << 1 >= dividend {
            value <<= 1;
            multiple <<= 1;
        }

        dividend -= value;
        result += multiple;
    }

    if negative { -result } else { result }
}

pub fn main() {
    let dividend = 10;
    let divisor = 3;
    println!("{}", divide(dividend, divisor));
}
