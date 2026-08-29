const MOD: i64 = 1_000_000_007;

fn max_sum_min_product(nums: Vec<i32>) -> i32 {
    let mut stack = Vec::new();
    let mut prefix = vec![0; nums.len()+2];
    let mut res = 0;
    let n = nums.len();
    for j in 0..=n {
        prefix[j+1] = *nums.get(j).unwrap_or(&0) as i64 + prefix[j];

        let i = j;
        while let Some(&idx) = stack.last() {
            if j == n || nums[idx] >= nums[j] {
                stack.pop();
                let total = prefix[j] - prefix[stack.last().map_or(0, |v| *v+1)];
                res = res.max(total * nums[idx] as i64);
            } else {
                break;
            }
        }

        stack.push(i);
    }

    (res % MOD) as i32
}

pub fn main() {
    let nums = [1,1,3,4];
    println!("{}", max_sum_min_product(nums.into()));
}
