fn closest_divisors(num: i32) -> Vec<i32> {
    let mut result = vec![0,num];

    let recursion = |num: i32, result: &mut [i32]| {
        for i in 2..=num.isqrt() {
            if num % i == 0 && (i-num/i).abs() < (result[0]-result[1]).abs() {
                result[0] = i;
                result[1] = num/i;
            }
        }
    };

    recursion(num+1, &mut result);
    recursion(num+2, &mut result);

    result
}

pub fn main() {
    let num = 123;
    println!("{:?}", closest_divisors(num));
}
