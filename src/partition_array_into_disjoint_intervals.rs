fn partition_disjoint(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut suffix = vec![nums[n-1]; n];
    for i in (0..n-1).rev() {
        suffix[i] = nums[i].min(suffix[i+1]);
    }

    let mut max = nums[0];
    for i in 0..nums.len()-1 {
        max = max.max(nums[i]);
        if max <= suffix[i+1] {
            return (i+1) as i32;
        }
    }

    unreachable!()
}

pub fn main() {
    let nums = [5,0,3,8,6].to_vec();
    println!("{}", partition_disjoint(nums));
}
