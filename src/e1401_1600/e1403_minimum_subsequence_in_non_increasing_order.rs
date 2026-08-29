fn min_subsequence(mut nums: Vec<i32>) -> Vec<i32> {
    nums.sort_unstable();

    let mut sum = nums.iter().sum::<i32>();
    let mut total = 0;
    let mut res = Vec::new();
    for num in nums.into_iter().rev() {
        res.push(num);
        total += num;
        sum -= num;
        if total>sum { break; }
    }

    res
}

pub fn main() {
    let nums = [4,3,10,9,8].to_vec();
    println!("{:?}", min_subsequence(nums));
}
