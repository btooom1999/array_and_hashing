fn sum_of_encrypted_int(nums: Vec<i32>) -> i32 {
    let mut res = 0;
    for mut num in nums {
        let mut max = 0;
        let mut factor = 0;
        while num > 0 {
            factor = factor * 10 + 1;
            max = max.max(num % 10);
            num /= 10;
        }

        res += factor * max
    }

    res
}

pub fn main() {
    let nums = [1,2,3].to_vec();
    println!("{}", sum_of_encrypted_int(nums));
}
