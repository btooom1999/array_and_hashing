fn longest_subarray(nums: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut res = 0;
    let mut count = 0;

    for num in nums {
        if num > max {
            count = 1;
            max = num;
            res = 1;
        } else if num < max {
            count = 0;
        } else {
            count += 1;
        }

        res = res.max(count);
    }

    res
}

pub fn main() {
    let nums = [1,2,3,3,2,2].to_vec();
    println!("{}", longest_subarray(nums));
}
