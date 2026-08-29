fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let n = heights.len();
    let mut left = vec![-1; n];
    let mut right = vec![n as i32; n];
    let mut stack = Vec::new();
    for i in 0..n {
        while let Some(&top) = stack.last() {
            if heights[top] >= heights[i] {
                right[top] = i as i32;
                stack.pop();
            } else {
                left[i] = top as i32;
                break;
            }
        }

        stack.push(i);
    }

    let mut res = 0;
    for i in 0..n {
        res = res.max(heights[i] * (right[i] - left[i] - 1));
    }

    res
}

pub fn main() {
    let heights = [2,1,5,6,2,3].to_vec();
    println!("{}", largest_rectangle_area(heights));
}
