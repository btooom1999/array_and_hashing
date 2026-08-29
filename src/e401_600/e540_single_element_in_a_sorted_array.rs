fn single_non_duplicate(nums: Vec<i32>) -> i32 {
    let mut l = 0;
    let mut r = nums.len() - 1;
    while l < r {
        let m = (l + r) / 2;
        let val1 = nums.get(m);
        let val2 = nums.get(m-1);
        let val3 = nums.get(m+1);
        if m.is_multiple_of(2) {
            if val1 == val3 {
                l = m + 1;
            } else {
                r = m;
            }
        } else if val1 == val2 {
            l = m + 1;
        } else {
            r = m;
        }
    }

    nums[l]
}

pub fn main() {
    let nums = [1,2,2,3,3,4,4,8,8].to_vec();
    // let nums = [3,3,7,7,10,11,11].to_vec();
    println!("{}", single_non_duplicate(nums));
}
