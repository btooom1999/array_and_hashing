fn maximum_count(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut l = 0;
    let mut r = n;
    while l < r {
        let m = (l+r)/2;
        if nums[m] >= 0 {
            r = m;
        } else {
            l = m+1;
        }
    }

    let negative_counts = l;
    l = 0;
    r = n;
    while l < r {
        let m = (l+r)/2;
        if nums[m] <= 0 {
            l = m+1;
        } else {
            r = m;
        }
    }

    negative_counts.max(n-l) as i32
}

pub fn main() {
    let nums = [-3,-2,-1,0,0,1,2].to_vec();
    println!("{}", maximum_count(nums));
}
