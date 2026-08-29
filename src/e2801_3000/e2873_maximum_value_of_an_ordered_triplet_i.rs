fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
    let mut res = 0;
    let mut max = 0;
    let mut distance = 0;

    for num in nums {
        let num = num as i64;
        res = res.max(distance * num);
        distance = distance.max(max - num);
        max = max.max(num);
    }

    res
}

pub fn main() {
    // let nums = [8,6,3,13,2,12,19,5,19,6,10,11,9].to_vec();
    let nums = [15,12,2,14,15,18,15,20,14,5,14,14,11,13,7].to_vec();
    println!("{}", maximum_triplet_value(nums));
}
