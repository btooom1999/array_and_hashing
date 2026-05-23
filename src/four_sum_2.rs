use std::collections::HashMap;

fn four_sum_count(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>, nums4: Vec<i32>) -> i32 {
    let mut hashmap1 = HashMap::<_, i32>::new();
    let mut hashmap3 = HashMap::<_, i32>::new();
    let n = nums1.len();

    for i in 0..n {
        *hashmap1.entry(nums1[i]).or_default() += 1;
        *hashmap3.entry(nums3[i]).or_default() += 1;
    }

    let mut hashmap2 = HashMap::<_, i32>::new();
    let mut hashmap4 = HashMap::<_, i32>::new();
    for i in 0..n {
        for (&k, &v) in &hashmap1 {
            *hashmap2.entry(k+nums2[i]).or_default() += v;
        }
        for (&k, &v) in &hashmap3 {
            *hashmap4.entry(k+nums4[i]).or_default() += v;
        }
    }

    let mut count = 0;
    for (k, count1) in hashmap2 {
        if let Some(count2) = hashmap4.remove(&-k) {
            count += count1 * count2;
        }
    }

    count
}

fn generate_random_numbers() -> Vec<i32> {
    let lower: i32 = -(1 << 28); // -2^28
    let upper: i32 = 1 << 28;    //  2^28

    (0..200)
        .map(|_| rand::random_range(lower..=upper))
        .collect()
}

pub fn main() {
    let nums1 = [1;200].to_vec();
    let nums2 = [-1;200].to_vec();
    let nums3 = [1;200].to_vec();
    let nums4 = [-1;200].to_vec();
    println!("{:?}", nums1);
    println!("{:?}", nums2);
    // let nums1 = generate_random_numbers();
    // let nums2 = generate_random_numbers();
    // let nums3 = generate_random_numbers();
    // let nums4 = generate_random_numbers();
    println!("{}", four_sum_count(nums1, nums2, nums3, nums4));
}
