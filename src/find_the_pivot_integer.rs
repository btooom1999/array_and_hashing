fn pivot_integer(n: i32) -> i32 {
    let sum = n*(n+1)/2;
    let mut current = 0;
    for num in 1..n+1 {
        if sum - current == current + num {
            return num;
        }
        current += num;
    }

    -1
}

pub fn main() {
    let n = 8;
    println!("{}", pivot_integer(n));
}
