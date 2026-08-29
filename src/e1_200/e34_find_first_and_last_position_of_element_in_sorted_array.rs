fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    if nums.is_empty() {
        return vec![-1, -1];
    }

    let mut l = 0;
    let mut r = nums.len() - 1;

    while l < r {
        let m = (l + r) / 2;
        if nums[m] < target {
            l = m + 1;
        } else {
            r = m;
        }
    }

    if nums[l] != target {
        return vec![-1, -1];
    }

    let mut res = vec![-1, -1];
    res[0] = l as i32;

    let mut l = 0;
    let mut r = nums.len() - 1;
    while l < r {
        let m = (l + r) / 2;
        if nums[m] <= target {
            l = m + 1;
        } else {
            r = m;
        }
    }

    res[1] = l as i32 - (nums[l] != target) as i32;

    res
}

pub fn main() {
    let nums = [5,7,7,7,7,7,7,7,7,8,8,10].to_vec();
    let target = 7;
    println!("{:?}", search_range(nums, target));
}
