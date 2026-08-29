fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut res = 0;

    if nums2.len() % 2 == 1 {
        res ^= nums1.iter().fold(0, |acc, num| acc ^ num);
    }

    if nums1.len() % 2 == 1 {
        res ^= nums2.iter().fold(0, |acc, num| acc ^ num);
    }

    res
}

pub fn main() {
    let nums1 = [2,1,3].to_vec();
    let nums2 = [10,2,5,0].to_vec();
    println!("{}", xor_all_nums(nums1, nums2));
}
