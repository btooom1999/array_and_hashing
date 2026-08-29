fn find_subarrays(nums: Vec<i32>) -> bool {
    let mut hashmap = std::collections::HashMap::<_, i32>::new();
    for i in 0..nums.len()-1 {
        let val = hashmap.entry(nums[i]+nums[i+1]).or_default();
        *val += 1;
        if *val == 2 { return true; }
    }

    false
}

pub fn main() {
    let nums = [4,2,4].to_vec();
    println!("{}", find_subarrays(nums));
}
