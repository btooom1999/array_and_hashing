use std::collections::HashMap;

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        (a, b) = (b, a % b);
    }

    a
}

fn interchangeable_rectangles(rectangles: Vec<Vec<i32>>) -> i64 {
    let mut hashmap = HashMap::<Vec<i32>, i64>::new();
    let mut res = 0i64;
    for rec in &rectangles {
        let mut a = rec[0];
        let mut b = rec[1];
        let gcd = gcd(a, b);
        a /= gcd;
        b /= gcd;
        let val = hashmap.entry(vec![a, b]).or_default();

        res += *val;
        *val += 1;
    }

    res
}

pub fn main() {
    let rectangles = vec![vec![4, 8], vec![3, 6], vec![10, 20], vec![15, 30]];
    println!("{}", interchangeable_rectangles(rectangles));
}
