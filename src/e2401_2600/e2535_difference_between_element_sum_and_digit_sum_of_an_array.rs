fn difference_of_sum(nums: Vec<i32>) -> i32 {
    let mut sum = 0;
    let mut digit = 0;

    for mut num in nums {
        sum += num;
        while num > 0 {
            digit += num % 10;
            num /= 10;
        }
    }

    (sum - digit).abs()
}

pub fn main() {
    let nums = [1,15,6,3];
    println!("{}", difference_of_sum(nums.into()));
}
