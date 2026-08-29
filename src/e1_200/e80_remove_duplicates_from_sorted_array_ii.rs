fn remove_duplicates(nums: &mut [i32]) -> i32 {
    if nums.len() <= 2 {
        return nums.len() as i32;
    }

    let mut w = 2;
    let mut total = 2;
    for r in 2..nums.len() {
        if nums[r] != nums[w-2] {
            nums[w] =  nums[r];
            w += 1;
            total += 1;
        }
    }

    total

}
pub fn main() {
    let mut nums = vec![0,0,0,1,1,1,2,2,2,3];
    // let mut nums = vec![0,0,1,1,1,1,2,3,3];
    // let mut nums = vec![1,1];

    println!("{}", remove_duplicates(&mut nums));
}
