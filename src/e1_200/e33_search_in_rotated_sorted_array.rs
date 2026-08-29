fn search(nums: Vec<i32>, target: i32) -> i32 {
    let n = nums.len() - 1;
    let mut l = 0;
    let mut r = n;

    if target == nums[n] {
        return n as i32;
    }

    while l < r {
        let m = (l + r) / 2;
        if nums[m] > nums[r] {
            l = m + 1;
        } else {
            r = m;
        }
    }

    let (mut r, mut l): (usize, usize) = if target > nums[n] {
        (std::cmp::max(l, 1) - 1, 0_usize)
    } else {
        (n, l)
    };

    while l < r {
        let m = (l + r) / 2;
        if nums[m] >= target {
            r = m;
        } else {
            l = m + 1;
        }
    }

    if nums[l] == target { l as i32 } else { -1 }
}

pub fn main() {
    let nums = [4,5,6,7,8,1,2].to_vec();
    // let nums = [1,2,3].to_vec();
    let target = 8;
    println!("{}", search(nums, target));
}
