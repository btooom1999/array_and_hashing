use std::collections::HashMap;

fn max_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut hashmap = HashMap::<i32, i32>::new();
    let n1 = nums1.len();
    let n2 = nums2.len();
    for i in 0..n1.max(n2) {
        if i < n1 {
            *hashmap.entry(nums1[i]).or_default() += 1;
        }
        if i < n2 {
            *hashmap.entry(nums2[i]).or_default() += 1;
        }
    }

    let mut i1 = 0;
    let mut i2 = 0;
    let mut sum1 = 0;
    let mut sum2 = 0;
    let mut total = 0;
    while i1 < n1 || i2 < n2 {
        while i1 < n1 && hashmap[&nums1[i1]] == 1 {
            sum1 += nums1[i1] as i64;
            i1 += 1;
        }
        while i2 < n2 && hashmap[&nums2[i2]] == 1 {
            sum2 += nums2[i2] as i64;
            i2 += 1;
        }

        total += sum1.max(sum2);
        if i1 < n1 {
            sum1 = nums1[i1] as i64;
            i1 += 1;
        } else {
            sum1 = 0;
        }
        if i2 < n2 {
            sum2 = nums2[i2] as i64;
            i2 += 1;
        } else {
            sum2 = 0;
        }
    }

    ((total + sum1.max(sum2)) % 1_000_000_007) as i32
}

pub fn main() {
    let nums1 = [2,4,5,8,10].to_vec();
    let nums2 = [4,6,8,9].to_vec();
    println!("{}", max_sum(nums1, nums2));
}
