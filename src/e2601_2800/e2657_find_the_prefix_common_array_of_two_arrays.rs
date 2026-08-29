fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let (mut mask_a, mut mask_b) = (0i64, 0i64);

    let n = a.len();
    let mut res = Vec::new();
    for i in 0..n {
        mask_a ^= 1 << a[i];
        mask_b ^= 1 << b[i];

        res.push((mask_a & mask_b).count_ones() as i32);
    }

    res
}

pub fn main() {
    let a = [1,3,2,4].to_vec();
    let b = [3,1,2,4].to_vec();
    println!("{:?}", find_the_prefix_common_array(a, b));
}
