fn is_perfect_square(num: i32) -> bool {
    let num = num as i64;
    let mut l = 0;
    let mut r = num;

    while l < r {
        let m = (l + r) / 2;
        let val = m * m;
        if val == num {
            return true;
        } else if val > num {
            r = m - 1;
        } else {
            l = m + 1;
        }
    }

    l * l == num
}

pub fn main() {
    let num = 14;
    println!("{}", is_perfect_square(num));
}
