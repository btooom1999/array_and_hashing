use std::collections::BTreeMap;

const N: i32 = 10_000;
fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut btreemap: BTreeMap<i32, Vec<i32>> = BTreeMap::new();
    let mut hashmap = vec![0; (N * 2 + 1) as usize];

    for num in &nums {
        hashmap[(*num + N) as usize] += 1;
    }

    for (i, count) in hashmap.iter().enumerate() {
        if *count == 0 {
            continue;
        }

        let vec= btreemap.entry(*count).or_default();
        if (vec.len() as i32) < k {
            vec.push(i as i32 - N);
        }
    }

    let mut need = k;
    let mut res = Vec::new();
    for value in btreemap.values().rev() {
        for num in value {
            res.push(*num);
            need -= 1;
            if need == 0 {
                return res
            }
        }
    }

    res

}

pub fn main () {
    // let nums = vec![1,1,1,2,2,3];
    // let nums  = vec![1];
    let nums = vec![1, 2, 2, 3, 3, 3];
    let k = 2;

    let res = top_k_frequent(nums, k);
    println!("{:?}", res);
}
