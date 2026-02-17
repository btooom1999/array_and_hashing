fn find_closest_number(nums: Vec<i32>) -> i32 {
    nums.into_iter().fold(i32::MAX, |res, num| {
        if num.abs() < res.abs()
            || (num.abs() == res.abs() && res < num) {
            num
        } else {
            res
        }
    })
}

pub fn main() {
    let nums = [-4,-2,1,4,8].to_vec();
    println!("{}", find_closest_number(nums));
}
