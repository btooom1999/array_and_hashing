fn search(nums: Vec<i32>, target: i32) -> bool {
    let n = nums.len() - 1;
    let mut l = 0;
    let mut r = n;

    if target == nums[n] {
        return true;
    }

    while l < r {
        let m = (l + r) / 2;
        if nums[m] == nums[r] {
            while r > 0 && nums[r] == nums[m] {
                r -= 1;
            }

            if nums[r] > nums[m] {
                l = r + 1;
            }
        } else if nums[m] > nums[r] {
            l = m + 1;
        } else {
            r = m;
        }
    }

    let (mut l, mut r): (usize, usize) = if target > nums[n] {
        (0, std::cmp::max(l, 1) - 1)
    } else {
        (l, n)
    };

    while l < r {
        let m = (l + r) / 2;
        if nums[m] == target {
            return true;
        } else if nums[m] < target {
            l = m + 1;
        } else {
            r = m;
        }
    }

    nums[l] == target
}

pub fn main() {
    // let nums = [2,5,6,0,0,1,2].to_vec();
    let nums =  [2,2,2,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,2,2,2,2,2,2,2,2,2,2,2].to_vec();
    let target = 1;
    println!("{}", search(nums, target));
}
