fn number_of_points(mut nums: Vec<Vec<i32>>) -> i32 {
    nums.sort_by(|a, b| a[0].cmp(&b[0]).then(b[1].cmp(&a[1])));
    let mut res = 0;
    let mut max = 0;
    let mut i = 0;
    for (j, pair) in nums.iter().enumerate() {
        if max < pair[0] {
            res += (max-nums[i][0]+1).max(0);
            i = j;
        }

        max = max.max(pair[1]);
    }

    res + max - nums[i][0] + 1
}

pub fn main() {
    let nums = [[3,6],[1,5],[4,7]].into_iter().map(Vec::from).collect();
    println!("{}", number_of_points(nums));
}
