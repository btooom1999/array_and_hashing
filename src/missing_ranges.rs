fn find_missing_ranges(
    nums: Vec<i32>,
    lower: i32,
    upper: i32,
) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    let nums = vec![lower-1].into_iter().chain(nums).chain(vec![upper+1]).collect::<Vec<_>>();
    for pair in nums.windows(2) {
        if pair[1] - pair[0] > 1 {
            res.push(vec![pair[0]+1, pair[1]-1]);
        }
    }

    res
}

pub fn main() {
    let nums = [0,1,3,50,75].to_vec();
    let lower = -5;
    let upper = 99;
    println!("{:?}", find_missing_ranges(nums, lower, upper));
}
