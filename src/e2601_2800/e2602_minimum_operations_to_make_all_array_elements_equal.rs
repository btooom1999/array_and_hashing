fn min_operations(mut nums: Vec<i32>, queries: Vec<i32>) -> Vec<i64> {
    nums.sort();

    let n = nums.len();
    let mut prefix = vec![0; n+1];
    for i in 0..n {
        prefix[i+1] = prefix[i] + nums[i] as i64;
    }

    let mut res = Vec::new();
    for target in queries {
        match nums.binary_search(&target) {
            Ok(i) => {
                let num = nums[i] as i64;
                let left = num * i as i64 - (prefix[i+1] - num);
                let right = prefix[n] - prefix[i+1] - num * (n-i-1) as i64;
                res.push(left+right);
            },
            Err(i) => {
                let left = target as i64 * i as i64 - prefix[i];
                let right = prefix[n] - prefix[i] - target as i64 * (n-i) as i64;
                res.push(left+right);
            },
        }
    }

    res
}

pub fn main() {
    let nums = [3,1,6,8].to_vec();
    let queries = [1,5,0].to_vec();
    println!("{:?}", min_operations(nums, queries));
}
