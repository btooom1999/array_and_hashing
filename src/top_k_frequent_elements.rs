use std::collections::HashMap;

fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut hashmap: HashMap<i32, i32> = HashMap::new();

    for num in nums.iter() {
        hashmap.entry(*num).and_modify(|v| *v +=1).or_insert(1);
    }

    let mut arr = hashmap.iter().collect::<Vec<_>>();
    arr.sort_by(|a, b| b.1.cmp(a.1));
    
    arr.into_iter().map(|t| t.0.to_owned()).collect::<Vec<_>>()[0..(k as usize)].to_owned()
}

pub fn main () {
    // let nums = vec![1,1,1,2,2,3];
    // let nums  = vec![1];
    let nums = vec![1,2,1,2,1,2,3,1,3,2];
    let k = 2;

    let res = top_k_frequent(nums, k);
    println!("{:?}", res);
}
