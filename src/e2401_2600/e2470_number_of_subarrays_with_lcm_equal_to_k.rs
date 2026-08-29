fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b > 0 {
        (a, b) = (b, a%b);
    }

    a
}

fn lcm(a: i32, b: i32) -> i32 {
    if let Some(prod) = a.checked_mul(b) {
        return prod / gcd(a, b);
    }

    -1
}

fn subarray_lcm(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let mut count = 0;
    for i in 0..n {
        let mut current_lcm = nums[i];
        for j in i..n {
            current_lcm = lcm(current_lcm, nums[j]);
            if current_lcm == -1 {
                break;
            }
            if current_lcm == k {
                count += 1;
            }
        }
    }

    count
}

pub fn main() {
    let nums = [2,3,5].to_vec();
    let k = 4;
    println!("{}", subarray_lcm(nums, k));
}
