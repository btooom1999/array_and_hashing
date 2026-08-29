fn increasing_triplet(nums: Vec<i32>) -> bool {
    let mut triplet = [i32::MAX, i32::MAX];

    for num in nums {
        if num <= triplet[0] {
            triplet[0] = num;
        } else if num <= triplet[1] {
            triplet[1] = num;
        } else {
            return true;
        }
    }

    false
}

pub fn main() {
    let nums = [1,2,3,4,5].to_vec();
    println!("{}", increasing_triplet(nums));
}
