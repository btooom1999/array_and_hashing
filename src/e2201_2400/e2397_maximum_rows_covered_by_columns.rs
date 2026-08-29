fn maximum_rows(matrix: Vec<Vec<i32>>, num_select: i32) -> i32 {
    let (m, n) = (matrix.len(), matrix[0].len());
    let mut res = 0;
    for num1 in (1i32<<num_select)-1..1i32<<(n+1) {
        if num_select == num1.count_ones() as i32 {
            let mut count = 0;
            for j in 0..m {
                if !matrix[j].iter().enumerate().any(|(i, &num2)| num2 == 1 && num1 >> (n-i-1) & 1 == 0) {
                    count += 1;
                }
            }

            res = res.max(count);
        }
    }

    res
}

pub fn main() {
    let matrix = [
        [0,0,0,1,0],
        [1,0,1,0,1],
        [0,1,1,1,0],
        [0,0,1,0,0],
        [1,1,1,1,1],
    ].into_iter().map(Vec::from).collect();
    let num_select = 3;
    println!("{}", maximum_rows(matrix, num_select));
}
