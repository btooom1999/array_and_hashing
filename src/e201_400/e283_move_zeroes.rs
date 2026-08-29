fn move_zeroes(nums: &mut [i32]) {
    let mut write = None as Option<usize>;
    for read in 0..nums.len() {
        if nums[read] == 0 && write.is_none() {
            write = Some(read);
        } else if nums[read] != 0 && let Some(w) = write {
            (nums[w], nums[read]) = (nums[read], nums[w]);
            write = Some(w + 1);
        }
    }
}

pub fn main() {
    let mut nums = vec![0,1,0,3,12];
    (move_zeroes(&mut nums));
    println!("{:?}", nums);
}
