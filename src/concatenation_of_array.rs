fn get_concatenation(mut nums: Vec<i32>) -> Vec<i32> {
    nums.append(&mut nums.clone());

    nums
}

pub fn main() {
    let nums = [1, 2, 3].to_vec();
    println!("{:?}", get_concatenation(nums));
}
