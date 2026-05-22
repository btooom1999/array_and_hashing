fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        (a, b) = (b, a % b);
    }

    a
}

fn lcm(a: i32, b: i32) -> i32 {
    a * b / gcd(a, b)
}

fn max_length(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut res = 0;
    for i in 0..n {
        let mut prod = 1;
        for j in i..n {
            prod *= nums[j];
            let gcd = nums[i..j+1].iter().copied().reduce(gcd).unwrap();
            let lcm = nums[i..j+1].iter().copied().reduce(lcm).unwrap();

            if gcd * lcm == prod {
                res = res.max(j-i+1);
            }
        }
    }

    res as i32
}

pub fn main() {
    let nums = [1,2,1,2,1,1,1].to_vec();
    println!("{}", max_length(nums));
}
