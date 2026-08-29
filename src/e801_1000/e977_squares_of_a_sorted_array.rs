fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let mut negative_nums = Vec::new();
    let mut positive_nums = Vec::new();
    for num in nums {
        if num < 0 {
            negative_nums.insert(0, num);
        } else {
            positive_nums.push(num);
        }
    }

    let mut i = 0;
    let mut j = 0;
    let mut res = Vec::new();
    while i < negative_nums.len() || j < positive_nums.len() {
        if j >= positive_nums.len() || (i < negative_nums.len() && negative_nums[i].abs() <= positive_nums[j]) {
            res.push(negative_nums[i] * negative_nums[i]);
            i += 1;
        } else if i >= negative_nums.len() || (j < positive_nums.len() && negative_nums[i].abs() > positive_nums[j]) {
            res.push(positive_nums[j] * positive_nums[j]);
            j += 1;
        }
    }

    res
}

pub fn main() {
    let nums = vec![-4,-1,0,3,10];
    println!("{:?}", sorted_squares(nums));
}
