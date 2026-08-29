fn sort_colors(nums: &mut Vec<i32>) {
    let mut sorted_nums = [Vec::new(), Vec::new(), Vec::new()];

    for num in nums.iter() {
        match num {
            0 => sorted_nums[0].push(0),
            1 => sorted_nums[1].push(1),
            2 => sorted_nums[2].push(2),
            _ => unreachable!(),
        }
    }

    *nums = sorted_nums.into_iter().flatten().collect::<Vec<_>>();
}

pub fn main() {
    let mut nums = vec![2, 0, 2, 1, 1, 0];
    sort_colors(&mut nums);
    println!("{:?}", nums);
}
