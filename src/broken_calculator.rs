fn broken_calc(start_value: i32, mut target: i32) -> i32 {
    let mut res = 0;
    while start_value != target {
        if start_value > target || target % 2 == 1 {
            target += 1;
        } else {
            target /= 2;
        }

        res += 1;
    }

    res
}

pub fn main() {
    let start_value = 5;
    let target = 8;
    println!("{}", broken_calc(start_value, target));
}
