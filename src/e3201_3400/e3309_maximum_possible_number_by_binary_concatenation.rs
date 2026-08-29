fn max_good_number(nums: Vec<i32>) -> i32 {
    let (n0, n1, n2) = (32-nums[0].leading_zeros(), 32-nums[1].leading_zeros(), 32-nums[2].leading_zeros());
    let mut res = 0;

    res = res.max((((nums[0] << n1) + nums[1]) << n2) + nums[2]);
    res = res.max((((nums[0] << n2) + nums[2]) << n1) + nums[1]);

    res = res.max((((nums[1] << n0) + nums[0]) << n2) + nums[2]);
    res = res.max((((nums[1] << n2) + nums[2]) << n0) + nums[0]);

    res = res.max((((nums[2] << n0) + nums[0]) << n1) + nums[1]);
    res = res.max((((nums[2] << n1) + nums[1]) << n0) + nums[0]);

    res
}

pub fn main() {
    let nums = [2,8,16].to_vec();
    println!("{}", max_good_number(nums));
}
