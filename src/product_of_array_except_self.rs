fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut pref: Vec<i32> = vec![1; nums.len()];

    for i in 1..nums.len() {
        pref[i] = pref[i-1] * nums[i-1];
    }

    let mut suff = 1;
    for i in (0..nums.len()).rev() {
        pref[i] *= suff;
        suff *= nums[i];
    }

    pref
}

pub fn main() {
    let nums = vec![1,2,3,4];
    //let nums = vec![-1,1,0,-3,3];

    let arr = product_except_self(nums);
    println!("{:?}", arr);
}
