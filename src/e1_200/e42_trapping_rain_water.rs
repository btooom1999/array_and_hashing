fn trap(height: Vec<i32>) -> i32 {
    let n = height.len();
    let mut suffix = vec![0; n+1];
    for i in (0..n).rev() {
        suffix[i] = height[i].max(suffix[i+1]);
    }

    let mut max = height[0];
    let mut res = 0;
    for i in 0..n {
        max = max.max(height[i]);
        res += (max.min(suffix[i+1]) - height[i]).max(0);
    }

    res
}

pub fn main() {
    let height = [0,1,0,2,1,0,1,3,2,1,2,1].to_vec();
    println!("{}", trap(height));
}
