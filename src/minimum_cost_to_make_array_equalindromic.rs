fn minimum_cost(mut nums: Vec<i32>) -> i64 {
    nums.sort();

    let mirror = |mut num: i32| -> i32 {
        let mut res = 0;
        while num > 0 {
            res *= 10;
            res += num % 10;
            num /= 10;
        }

        res
    };

    let mut l = nums[nums.len()/2];
    let mut r = l;

    while mirror(l) != l { l-= 1; }
    while mirror(r) != r { r+= 1; }

    let mut res = (0,0);
    for num in nums {
        res.0 += (l - num).abs() as i64;
        res.1 += (r - num).abs() as i64;
    }

    res.0.min(res.1)
}

pub fn main() {
    let nums = [301,309,312,322].to_vec();
    println!("{}", minimum_cost(nums));
}
