use std::collections::HashSet;

fn longest_consecutive(nums: Vec<i32>) -> i32 {
    if nums.is_empty() { return 0; }

    let mut hashset: HashSet<i32> = HashSet::with_capacity(nums.len()); 
    let mut longest = 1;

    for num in nums.iter() {
        hashset.insert(*num);
    }

    for num in nums.into_iter() {
        if hashset.contains(&num) && !hashset.contains(&(num - 1)) {
            let mut temp_longest = 1;
            let mut j = num + 1;
            while hashset.contains(&j) {
                hashset.remove(&j);
                j += 1;
                temp_longest += 1;
            }

            longest = std::cmp::max(longest, temp_longest);
        }
    }

    longest
}

pub fn main() {
    // let nums = vec![100,4,200,1,3,2];
    // let nums = vec![0,3,7,2,5,8,4,6,0,1];
    let nums = vec![0, -1];

    let res = longest_consecutive(nums);
    println!("{}", res);
}
