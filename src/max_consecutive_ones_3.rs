fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
    let mut l = 0;
    let mut k = k;
    let mut res = 0;

    for (r, &num) in nums.iter().enumerate() {
        if num == 0 {
            k -= 1;
        }

        while k < 0 {
            if nums[l] == 0 {
                k = 0;
            }
            l += 1;
        }

        res = res.max(r - l + 1);
    }

    res as i32
}

pub fn main() {
    let nums = [1,1,1,0,0,0,1,1,1,1,0].to_vec();
    let k = 2;
    println!("{}", longest_ones(nums, k));
}
