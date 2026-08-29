fn min_moves(mut target: i32, mut max_doubles: i32) -> i32 {
    let mut res = 0;
    while max_doubles > 0 && target > 1 {
        if target % 2 == 0 {
            target /= 2;
            max_doubles -= 1;
        } else {
            target -= 1;
        }
        res += 1;
    }

    res + target - 1
}

pub fn main() {
    let target = 5;
    let max_doubles = 0;
    println!("{}", min_moves(target, max_doubles));
}
