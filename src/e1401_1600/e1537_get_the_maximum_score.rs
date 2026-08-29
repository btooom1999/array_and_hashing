fn max_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let n1 = nums1.len();
    let n2 = nums2.len();

    let mut i1 = 0;
    let mut i2 = 0;
    let mut sum1 = 0;
    let mut sum2 = 0;
    let mut total = 0;
    while i1 < n1 && i2 < n2 {
        if nums1[i1] > nums2[i2] {
            sum2 += nums2[i2] as i64;
            i2 += 1;
        } else if nums1[i1] < nums2[i2] {
            sum1 += nums1[i1] as i64;
            i1 += 1;
        } else {
            total += sum1.max(sum2);
            sum1 = *nums1.get(i1).unwrap_or(&i32::MIN) as i64;
            sum2 = *nums2.get(i2).unwrap_or(&i32::MIN) as i64;
            i1 += 1;
            i2 += 1;
        }
    }

    while i1 < n1 {
        sum1 += nums1[i1] as i64;
        i1 += 1;
    }

    while i2 < n2 {
        sum2 += nums2[i2] as i64;
        i2 += 1;
    }

    ((total + sum1.max(sum2)) % 1_000_000_007) as i32
}

pub fn main() {
    let nums1 = [2,4,5,8,10].to_vec();
    let nums2 = [4,6,8,9].to_vec();
    println!("{}", max_sum(nums1, nums2));
}
