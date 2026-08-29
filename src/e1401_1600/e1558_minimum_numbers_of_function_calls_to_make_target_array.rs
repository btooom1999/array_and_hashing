fn min_operations(nums: Vec<i32>) -> i32 {
    let mut odd = 0;
    let mut even = 0;

    for mut num in nums {
        let mut temp = 0;
        while num > 0 {
            if num % 2 != 0 {
                odd += 1;
            }
            if num > 1 {
                temp += 1;
            }
            num >>= 1;
        }

        even = even.max(temp);
    }

    odd + even
}

pub fn main() {
    let nums = [4,2,5].to_vec();
    println!("{}", min_operations(nums));
}
