fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
    nums.sort_unstable();

    let mut closest = nums[0] + nums[1] + nums[2];
    let n = nums.len();
    for i in 0..n-2 {
        let mut left = i+1;
        let mut right = n-1;

        while left < right {
            let sum = nums[i] + nums[left] + nums[right];
            if (closest - target).abs() > (sum - target).abs() {
                closest = sum;
            }
            if sum < target {
                left += 1;
            } else if sum > target {
                right -= 1;
            } else {
                return sum;
            }
        }
    }

    closest
}

fn generate_nums() -> Vec<i32> {
    (0..500).map(|_| rand::random_range(-1000..=1000)).collect()
}

pub fn main() {
    // let nums = [4,0,5,-5,3,3,0,-4,-5].to_vec();
    // let target = -2;
    let nums = generate_nums();
    println!("{:?}", nums);
    let target = 300;
    println!("{}", three_sum_closest(nums, target));
}
