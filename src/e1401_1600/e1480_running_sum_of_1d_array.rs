fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut sum = 0;
    let mut res = Vec::with_capacity(nums.len());
    for num in &nums {
        sum += num;
        res.push(sum);
    }

    res
}

pub fn main() {
    let nums = vec![1, 2, 3, 4];
    println!("{:?}", running_sum(nums));
}
