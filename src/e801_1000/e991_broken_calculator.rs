fn broken_calc(start_value: i32, mut target: i32) -> i32 {
    let mut res = 0;
    while start_value < target {
        if target % 2 == 0 {
            target /= 2;
        } else {
            target += 1;
        }

        res += 1;
    }

    res + start_value - target
}

pub fn main() {
    let start_value = 5;
    let target = 8;
    println!("{}", broken_calc(start_value, target));
}
