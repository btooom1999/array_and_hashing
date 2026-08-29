fn gcd(mut x: i32, mut y: i32) -> i32 {
    while y != 0 {
        (x, y) = (y, x % y);
    }

    x
}

fn replace_non_coprimes(nums: Vec<i32>) -> Vec<i32> {
    let mut stack = Vec::new();

    for num in nums {
        let mut num = num;
        while let Some(&last) = stack.last() {
            let gcd = gcd(num, last);
            if gcd > 1 {
                stack.pop();
                num = (num as i64 * last as i64 / gcd as i64) as i32;
            } else {
                break;
            }

        }
        stack.push(num);
    }

    stack
}

pub fn main() {
    // let nums = vec![6,4,3,2,7,6,2];
    let nums = vec![10, 21, 11, 33, 14, 5];
    println!("{:?}", replace_non_coprimes(nums));
}
