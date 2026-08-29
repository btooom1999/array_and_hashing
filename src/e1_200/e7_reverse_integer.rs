fn reverse(mut x: i32) -> i32 {
    if x == i32::MIN || x == i32::MAX {
        return 0;
    }

    let mut mask = 0;
    let sign = x.signum();
    x *= sign;
    while x > 0 {
        let mut a = mask << 3;
        let mut b = mask << 1;

        while a > 0 && b != 0 {
            let carry = (a & b) << 1;
            a ^=b;
            b = carry;
        }

        if a < 0 {
            return 0;
        }

        mask = a + x % 10;
        x /= 10;
    }

    mask*sign
}

pub fn main() {
    let x = 1534236469;
    println!("{}", reverse(x));
}
