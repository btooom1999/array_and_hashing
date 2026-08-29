fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
    let mut mask = 0u128;
    let mut res = Vec::new();

    let mut a = vec![None; 101];
    let mut b = vec![None; 101];
    let mut c = vec![None; 101];
    for num in nums1 {
        a[num as usize] = Some(num);
    }
    for num in nums2 {
        b[num as usize] = Some(num);
    }
    for num in nums3 {
        c[num as usize] = Some(num);
    }

    for num in a.into_iter().flatten().chain(b.into_iter().flatten().chain(c.into_iter().flatten())) {
        let bit = 1 << num as u128;
        if mask & bit > 0 {
            mask ^= bit;
            res.push(num);
        } else {
            mask |= bit;
        }
    }

    res
}

pub fn main() {
    let nums1 = [1,1,3,2].to_vec();
    let nums2 = [2,3].to_vec();
    let nums3 = [3].to_vec();
    println!("{:?}", two_out_of_three(nums1, nums2, nums3));
}
