fn min_flips(a: i32, b: i32, c: i32) -> i32 {
    let mut count = 0;
    for i in 0..32 {
        if c >> i & 1 == 1 {
            if a >> i & 1 == 0 && b >> i & 1 == 0 {
                count += 1;
            }
        } else {
            if a >> i & 1 == 1 {
                count += 1;
            }
            if b >> i & 1 == 1 {
                count += 1;
            }
        }
    }

    count
}

pub fn main() {
    let a = 2;
    let b = 6;
    let c = 5;
    println!("{}", min_flips(a, b, c));
}
