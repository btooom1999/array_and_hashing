fn last_remaining(n: i32) -> i32 {
    let mut head = 1;
    let mut step = 1;
    let mut from_left = true;
    let mut remaining = n;

    while remaining > 1 {
        if from_left || remaining % 2 == 1 {
            head += step;
        }

        step *= 2;
        remaining /= 2;
        from_left = !from_left;
    }

    head
}

pub fn main() {
    let n = 1_000_000_000;
    println!("{}", last_remaining(n));
}
