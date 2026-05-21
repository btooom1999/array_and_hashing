fn find_gcd(nums: Vec<i32>) -> i32 {
    let mut min = i32::MAX;
    let mut max = i32::MIN;

    for &num in &nums {
        min = min.min(num);
        max = max.max(num);
    }

    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 { return a; }
        gcd(b, a%b)
    }

    gcd(min, max)
}

pub fn main() {
    let nums = [2,5,6,9,10].to_vec();
    println!("{}", find_gcd(nums));
}
