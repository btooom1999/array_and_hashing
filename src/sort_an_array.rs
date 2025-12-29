fn merge(nums: &mut [i32], l: i32, m: i32, r: i32) {
    let n1 = m - l + 1;
    let n2 = r - m;

    let mut L = vec![0; n1 as usize];
    let mut R = vec![0; n2 as usize];

    for i in 0..n1 {
        L[i as usize] = nums[(l + i) as usize];
    }
    for j in 0..n2 {
        R[j as usize] = nums[(m + 1 + j) as usize];
    }

    let mut i = 0;
    let mut j = 0;
    let mut k = l;

    while i < n1 && j < n2 {
        if L[i as usize] <= R[j as usize] {
            nums[k as usize] = L[i as usize];
            i += 1;
        } else {
            nums[k as usize] = R[j as usize];
            j += 1;
        }
        k += 1;
    }

    while i < n1 {
        nums[k as usize] = L[i as usize];
        i += 1;
        k += 1;
    }

    while j < n2 {
        nums[k as usize] = R[j as usize];
        j += 1;
        k += 1;
    }
}
fn merge_sort(nums: &mut Vec<i32>, l: i32, r: i32) {
    if l >= r {
        return;
    }

    let m = (l + r) / 2;

    merge_sort(nums, l, m);
    merge_sort(nums, m + 1, r);
    merge(nums, l, m, r);
}

fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
    let r = nums.len() as i32 - 1;
    let l = 0;
    merge_sort(&mut nums, l, r);

    nums.clone()
}

pub fn main() {
    let nums = vec![5, 2, 3, 1];
    println!("{:?}", sort_array(nums));
}
