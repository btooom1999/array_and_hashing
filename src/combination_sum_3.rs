use std::collections::HashSet;

fn dfs(
    idx: i32,
    target: i32,
    k: usize,
    nums: &mut Vec<i32>,
    result: &mut HashSet<Vec<i32>>,
) {
    if idx == 10 || nums.len() == k {
        if target == 0 && nums.len() == k {
            result.insert(nums.clone());
        }
        return;
    }

    dfs(idx+1, target, k, nums, result);

    for i in idx..10 {
        if target-i < 0 {
            break;
        }

        nums.push(i);
        dfs(i+1, target-i, k, nums, result);
        nums.pop();
    }
}

fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
    let mut result = HashSet::new();
    dfs(1, n, k as usize, &mut vec![], &mut result);

    result.into_iter().collect()
}

pub fn main() {
    let k = 3;
    let n = 7;
    println!("{:?}", combination_sum3(k, n));
}
