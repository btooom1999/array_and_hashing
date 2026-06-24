use std::collections::HashMap;

fn get_distances(nums: Vec<i32>) -> Vec<i64> {
    let n = nums.len();
    let mut prefix = HashMap::<i32, (i64, i64, i64, i64)>::new();
    for i in 0..n {
        let val = prefix.entry(nums[i]).or_default();
        val.0 += i as i64;
        val.1 += 1;
    }

    let mut res = Vec::new();
    for i in 0..n {
        let value = prefix.entry(nums[i]).or_default();
        if value.1 > 0 {
            value.0 -= i as i64;
            value.1 -= 1;
        }

        res.push((i as i64 * value.3 - value.2).abs() + (i as i64 *value.1 - value.0).abs());

        value.2 += i as i64;
        value.3 += 1;
    }

    res
}

pub fn main() {
    let nums = [1,3,1,1,2].to_vec();
    println!("{:?}", get_distances(nums));
}
