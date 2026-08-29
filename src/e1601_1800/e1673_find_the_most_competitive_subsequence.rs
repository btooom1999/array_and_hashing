fn most_competitive(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let n = nums.len();

    let mut res = nums[..k].to_vec();
    let mut stack = Vec::new();
    for (i, num) in nums.into_iter().enumerate() {
        while let Some(&last) = stack.last() && last > num {
            if stack.len()-1+n-i < k {
                break;
            }

            stack.pop();
        }

        stack.push(num);
        if stack.len() == k {
            res = res.min(stack.clone());
        }
    }

    res
}

pub fn main() {
    let nums = [2,4,3,3,5,4,9,6].to_vec();
    let k = 4;
    println!("{:?}", most_competitive(nums, k));
}
