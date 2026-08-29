fn min_bit_flips(mut start: i32, mut goal: i32) -> i32 {
    let mut res = 0;
    while start > 0 || goal > 0 {
        if 1 & start != 1 & goal {
            res += 1;
        }

        start >>= 1;
        goal >>= 1;
    }

    res
}

pub fn main() {
    let start = 10;
    let goal = 7;
    println!("{}", min_bit_flips(start, goal));
}
