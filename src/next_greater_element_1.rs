fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut res = Vec::new();

    for num1 in nums1.iter() {
        let mut greater_num1 = -1;
        let mut flag = false;
        for num2 in nums2.iter() {
            if num1 == num2 {
                flag = true;
                continue;
            }

            if flag && num2 > num1 {
                greater_num1 = *num2;
                break;
            }
        }

        res.push(greater_num1);
    }

    res
}

pub fn main() {
    let nums1 = vec![4, 1, 2];
    let nums2 = vec![1, 3, 4, 2];
    println!("{:?}", next_greater_element(nums1, nums2));
}
