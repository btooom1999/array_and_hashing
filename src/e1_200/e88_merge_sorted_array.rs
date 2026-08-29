fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let (mut i, mut j, mut k) = (m - 1, n - 1, m + n - 1);
    while i >= 0 || j >= 0 {
        if i < 0 {
            nums1[k as usize] = nums2[j as usize];
            j -= 1;
        } else if j < 0 {
            nums1[k as usize] = nums1[i as usize];
            i -= 1;
        }
        else if nums1[i as usize] > nums2[j as usize] {
            nums1[k as usize] = nums1[i as usize];
            i -= 1;
        }
        else {
            nums1[k as usize] = nums2[j as usize];
            j -= 1;
        }

        k -= 1;
    }
}

pub fn main() {
    let mut nums1 = vec![1,2,3,0,0,0];
    let m = 3;
    let mut nums2 = vec![2,5,6];
    let n = 3;
    merge(&mut nums1, m, &mut nums2, n);
    println!("{:?}", nums1);
}
