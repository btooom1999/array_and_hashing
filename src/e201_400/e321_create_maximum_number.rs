fn get_max_seq(nums: &[i32], k: usize) -> Vec<i32> {
    let mut stack = Vec::new();
    for (i, &num) in nums.iter().enumerate() {
        while let Some(&last) = stack.last() && last < num {
            if stack.len()-1+nums.len()-i < k {
                break;
            }
            stack.pop();
        }
        stack.push(num);
    }

    stack
}

fn merge_sort(nums1: Vec<i32>, nums2: Vec<i32>, k: usize) -> Vec<i32> {
    let mut res = Vec::new();
    let mut i = 0;
    let mut j = 0;

    while (i < nums1.len() || j < nums2.len()) && res.len() < k {
        if nums1[i..] > nums2[j..] {
            res.push(nums1[i]);
            i += 1;
        } else {
            res.push(nums2[j]);
            j += 1;
        }
    }

    res
}

fn divide_and_conquer(nums1: &[i32], nums2: &[i32], k: usize) -> Vec<i32> {
    let mut res = vec![0; k];
    for i in 0..=k {
        if i <= nums1.len() && k-i <= nums2.len() {
            let seq1 = get_max_seq(nums1, i);
            let seq2 = get_max_seq(nums2, k-i);
            res = res.max(merge_sort(seq1,seq2, k));
        }
    }

    res
}

fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    divide_and_conquer(&nums1, &nums2, k)
}

pub fn main() {
    let nums1 = [6,7].to_vec();
    let nums2 = [6,0,4].to_vec();
    let k = 5;
    println!("{:?}", max_number(nums1, nums2, k));
}
