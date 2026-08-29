fn max_area(height: Vec<i32>) -> i32 {
    let mut l = 0;
    let mut r = height.len() - 1;
    let mut res = 0;
    while l < r {
        res = std::cmp::max(res, (r - l) as i32 * (std::cmp::min(height[l], height[r])),);
        if height[l] <= height[r] {
            l += 1;
        } else {
            r -= 1;
        }
    }

    res
}

pub fn main() {
    let height = vec![1,8,6,2,5,4,8,3,7];
    println!("{}", max_area(height));
}
