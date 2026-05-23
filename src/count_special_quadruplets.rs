fn count_quadruplets(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut count = 0;
    for i in 0..n {
        for j in i+1..n {
            for x in j+1..n {
                for y in x+1..n {
                    if nums[i] + nums[j] + nums[x] == nums[y] {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

pub fn main() {
    let nums = [1;50].to_vec();
    println!("{}", count_quadruplets(nums));
}
