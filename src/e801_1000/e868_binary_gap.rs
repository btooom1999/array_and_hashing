fn binary_gap(mut n: i32) -> i32 {
    let mut res = 0;
    let mut num = 0;
    let mut distance = 0;
    while n > 0 {
        distance += 1;
        if (1 & n) == 1 {
            if num == 1 {
                res = res.max(distance);
            }

            num = 1;
            distance = 0;
        }
        n >>= 1;
    }

    res
}

pub fn main() {
    let n = 22;
    println!("{}", binary_gap(n));
}
