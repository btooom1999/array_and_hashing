fn backtracking(
    index: usize,
    nums: &Vec<i32>,
    mut vec: Vec<i32>,
    result: &mut Vec<Vec<i32>>,
) {
    if index == nums.len() {
        result.push(vec);
        return;
    }

    backtracking(index+1, nums, vec.clone(), result);

    vec.push(nums[index]);
    backtracking(index+1, nums, vec, result);
}

fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    backtracking(0, &nums, vec![], &mut result);

    result
}

pub fn main() {
    let nums = [1,2,3];
    println!("{:?}", subsets(nums.into()));
}
