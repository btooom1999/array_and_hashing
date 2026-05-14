fn max_distance(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut j = nums2.len()-1;
    let mut res = 0;
    for i in (0..nums1.len()).rev() {
        while j > i && nums1[i] > nums2[j] {
            j -= 1;
        }

        res = res.max(j.saturating_sub(i));
    }

    res as i32
}

pub fn main() {
    let nums1 = [9819,9508,7398,7347,6337,5756,5493,5446,5123,3215,1597,774,368,313].to_vec();
    let nums2 = [9933,9813,9770,9697,9514,9490,9441,9439,8939,8754,8665,8560].to_vec();
    println!("{}", max_distance(nums1, nums2));
}
