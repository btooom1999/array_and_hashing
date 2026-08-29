fn min_operations(n: i32) -> i32 {
    if n & (n-1) == 0 {
        return 1;
    }

    let power = n & -n;
    1 + min_operations(n + power).min(min_operations(n - power))
}

pub fn main() {
    let n = 668;
    println!("{}", min_operations(n));
}
