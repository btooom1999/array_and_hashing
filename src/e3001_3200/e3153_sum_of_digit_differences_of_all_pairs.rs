fn sum_digit_differences(mut nums: Vec<i32>) -> i64 {
    let n = nums.len() as i64;
    let mut total = 0;

    while nums[0] > 0 {
        let mut counts = [0i64; 10];
        for num in &mut nums {
            counts[(*num % 10) as usize] += 1;
            *num /= 10;
        }

        let mut remaining = n;
        for count in counts {
            if count > 0 {
                remaining -= count;
                total += remaining * count;
            }
        }
    }

    total
}

pub fn main() {
    let nums = [123,456,789].to_vec();
    println!("{}", sum_digit_differences(nums));
}
