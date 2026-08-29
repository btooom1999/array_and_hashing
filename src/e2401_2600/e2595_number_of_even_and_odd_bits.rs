fn even_odd_bit(mut n: i32) -> Vec<i32> {
    let mut res = vec![0, 0];
    let mut pos = 0;
    while n > 0 {
        if (1 & n) == 1 {
            res[pos % 2] += 1;
        }
        n >>= 1;
        pos += 1;
    }

    res
}

pub fn main() {
    let n = 50;
    println!("{:?}", even_odd_bit(n));
}
