fn smallest_divisor(nums: Vec<i32>, threshold: i32) -> i32 {
    let mut l = 1;
    let mut r = *nums.iter().max().unwrap();

    while l <= r {
        let m = (l + r) / 2;
        let mut sum = 0;
        for &num in &nums {
            sum += (num as f32 / m as f32).ceil() as i32;
            if sum > threshold {
                break;
            }
        }

        if sum <= threshold {
            r = m - 1;
        } else {
            l = m + 1;
        }
    }

    l
}

pub fn main() {
    let nums = [1,2,5,9].to_vec();
    let threshold = 6;
    println!("{}", smallest_divisor(nums, threshold));
}
