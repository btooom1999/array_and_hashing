fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let (n1, n2) = (nums1.len(), nums2.len());
    let mut res = 0;
    for i in 0..n1 {
        let mut x = i;
        for j in 0..n2 {
            if x == n1 {
                break;
            }
            if nums1[x] == nums2[j] {
                res = res.max(x-i+1);
                x += 1;
            } else if nums1[i] == nums2[j] {
                res = res.max(1);
                x = i+1;
            } else {
                x = i;
            }
        }
    }

    for i in 0..n2 {
        let mut x = i;
        for j in 0..n1 {
            if x == n2 {
                break;
            }
            if nums2[x] == nums1[j] {
                res = res.max(x-i+1);
                x += 1;
            } else if nums2[i] == nums1[j] {
                res = res.max(1);
                x = i+1;
            } else {
                x = i;
            }
        }
    }

    res as i32
}

pub fn main() {
    let nums1 = [0; 1000].to_vec();
    let nums2 = [0; 1000].to_vec();
    // let nums1 = [1,2,3,2,1].to_vec();
    // let nums2 = [3,2,1,4,7].to_vec();
    println!("{}", find_length(nums1, nums2));
}
