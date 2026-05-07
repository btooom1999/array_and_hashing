fn max_alternating_sum(nums: Vec<i32>) -> i64 {
    let mut max_even = 0;
    let mut max_odd = 0;

    for num in nums {
        let num = num as i64;
        let new_max_even = max_odd + num;
        max_odd = max_odd.max(max_even-num);
        max_even = max_even.max(new_max_even);
    }

    max_even
}

pub fn main() {
    // let nums = [4,2,5,3].to_vec();
    let nums = [1,2,1,2,1,2,10,1,10,5,8,2,9,3,6].to_vec();
    println!("{}", max_alternating_sum(nums));
}
