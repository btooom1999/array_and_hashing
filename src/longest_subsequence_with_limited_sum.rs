fn answer_queries(mut nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
    nums.sort();

    let mut res = vec![0; queries.len()];
    for (i, q) in queries.iter().enumerate() {
        let mut sum = 0;
        for (j, num) in nums.iter().enumerate() {
            sum += *num;
            if sum > *q {
                break;
            }
            res[i] += 1;
        }
    }

    res
}

pub fn main() {
    let nums = vec![4, 5, 2, 1];
    let queries = vec![3, 10, 21];
    println!("{:?}", answer_queries(nums, queries));
}
