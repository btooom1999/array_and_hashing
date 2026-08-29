
fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    let mut i = 0;
    let mut j = 0;
    while i < nums1.len() || j < nums2.len() {
        if let (Some(vec1), Some(vec2)) = (nums1.get(i), nums2.get(j)) {
            if vec1[0] < vec2[0] {
                res.push(vec1.to_owned());
                i += 1;
            } else if vec1[0] > vec2[0] {
                res.push(vec2.to_owned());
                j += 1;
            } else {
                res.push(vec![vec1[0], vec1[1] + vec2[1]]);
                i += 1;
                j += 1;
            }
        } else if nums1.get(i).is_none() {
            res.push(nums2[j].clone());
            j += 1;
        } else {
            res.push(nums1[i].clone());
            i += 1;
        }
    }

    res
}

pub fn main() {
    let nums1 = vec![vec![1,2],vec![2,3],vec![4,5]];
    let nums2 = vec![vec![1,4],vec![3,2],vec![4,1]];
    println!("{:?}", merge_arrays(nums1, nums2));
}
