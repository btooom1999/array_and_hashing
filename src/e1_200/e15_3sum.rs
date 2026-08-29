
fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort();

    let mut res = Vec::new();
    for (i, num) in nums.iter().take(nums.len() - 2).enumerate() {
        if i > 0 && *num == nums[i-1] {
            continue;
        }

        let mut l = i + 1;
        let mut r =  nums.len() - 1;
        while l < r {
            let sum = *num + nums[l] + nums[r];
            if sum < 0 {
                l += 1;
            } else if sum > 0 {
                r -= 1;
            } else {
                let val = vec![*num, nums[l], nums[r]];
                while l < r && val[1] == nums[l] {
                    l += 1;
                }
                while l < r && val[2] == nums[r] {
                    r -= 1;
                }
                res.push(val);
            }
        }
    }

    res
}

pub fn main() {
    let nums = vec![-1,0,1,0];
    println!("{:?}", three_sum(nums));
}
