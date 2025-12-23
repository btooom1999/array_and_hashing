fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut pos = 0;
    let mut n = 0;

    for i in 0..nums.len() {
        if nums[i] != val {
            nums.swap(i, pos);
            pos += 1;
            n += 1;
        }
    }

    n
}

pub fn main() {
    //let mut nums = vec![3, 3, 2, 2];
    let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
    let val = 2;
    println!("{}", remove_element(&mut nums, val));
}
