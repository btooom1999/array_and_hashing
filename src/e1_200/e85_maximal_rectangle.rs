fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
    let n = matrix[0].len();
    let mut heights = vec![0; n];
    let mut ans = 0;

    for row in matrix {
        for j in 0..n {
            if row[j] == '1' {
                heights[j] += 1;
            } else {
                heights[j] = 0;
            }
        }
        ans = ans.max(largest_rectangle_area(&heights));
    }

    ans
}

fn largest_rectangle_area(heights: &[i32]) -> i32 {
    let mut res = 0;
    let n = heights.len();
    let mut stack: Vec<usize> = Vec::new();
    let mut left = vec![-1; n];
    let mut right = vec![n as i32; n];

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

    for i in 0..n {
        res = res.max(heights[i] * (right[i] - left[i] - 1));
    }

    res
}

pub fn main() {
    // let matrix = [["1","0","1","0","0"],["1","0","1","1","1"],["1","1","1","1","1"],["1","0","0","1","0"]]
    let matrix = [["0", "1", "0", "0"], ["1", "1", "1", "1"], ["1", "1", "1", "1"]]
        .into_iter()
        .map(|v| v.into_iter().map(|v| v.chars().next().unwrap()).collect())
        .collect();
    println!("{}", maximal_rectangle(matrix));
}
