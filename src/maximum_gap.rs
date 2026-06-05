fn maximum_gap(nums: Vec<i32>) -> i32 {
    let mut buckets: [Vec<(i32, i32)>; 10] = std::array::from_fn(|_| Vec::new());
    for &num in &nums {
        buckets[(num%10) as usize].push((num/10, num));
    }

    while buckets[0].len() != nums.len() || buckets[0].iter().any(|v| v.0 > 0) {
        let mut temp_buckets: [Vec<(i32, i32)>; 10] = std::array::from_fn(|_| Vec::new());
        for nums in &buckets {
            for &(divisor, num) in nums {
                temp_buckets[(divisor%10) as usize].push((divisor/10, num));
            }
        }
        buckets = temp_buckets;
    }

    let mut max = 0;
    for i in 1..buckets[0].len() {
        max = max.max(buckets[0][i].1-buckets[0][i-1].1);
    }

    max
}

pub fn main() {
    let nums = [1,11,111,3,33,6,66,9,99].to_vec();
    println!("{}", maximum_gap(nums));
}
