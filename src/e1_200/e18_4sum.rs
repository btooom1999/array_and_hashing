fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    nums.sort();

    let mut res = Vec::new();
    for (i, num) in nums.iter().take(nums.len() - 3).enumerate() {
        if i > 0 && *num == nums[i-1] {
            continue;
        }

        let mut j = nums.len() - 1;
        while j - i >= 3 {
            let mut l = i + 1;
            let mut r = j - 1;
            while l < r {
                let sum = *num as i64 + nums[l] as i64 + nums[r] as i64 + nums[j] as i64;
                if sum < target as i64 {
                    l += 1;
                } else if sum > target as i64 {
                    r -= 1;
                } else {
                    let quadruplet = vec![*num, nums[l], nums[r], nums[j]];
                    while l < r && quadruplet[1] == nums[l] {
                        l += 1;
                    }
                    while l < r && quadruplet[2] == nums[r] {
                        r -= 1;
                    }

                    res.push(quadruplet);
                }
            }
            let val = nums[j];
            while j - i >= 3 && val == nums[j] {
                j -= 1;
            }
        }
    }

    res
}

pub fn main() {
    let nums = vec![1,0,-1,0,-2,2];
    let target = 0;
    println!("{:?}", four_sum(nums, target));
}
