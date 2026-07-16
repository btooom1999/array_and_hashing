fn rotate_the_box(box_grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let (m, n) = (box_grid.len(), box_grid[0].len());
    let mut res = vec![vec!['a'; m]; n];

    for i in 0..m {
        for j in 0..n {
            res[j][m-i-1] = box_grid[i][j];
        }
    }

    for j in 0..m {
        let mut empty_i = n;
        for i in (0..n).rev() {
            if empty_i == n && res[i][j] == '.' {
                empty_i = i;
            } else if res[i][j] == '#' && empty_i != n {
                res[i][j] = '.';
                res[empty_i][j] = '#';
                empty_i -= 1;
            } else if res[i][j] == '*' {
                empty_i = n;
            }
        }
    }

    res
}

pub fn main() {
    let box_grid = [["#","#","*",".","*","."], ["#","#","#","*",".","."], ["#","#","#",".","#","."]]
        .into_iter()
        .map(|v| v.into_iter().map(|v| v.chars().next().unwrap()).collect())
        .collect();
    println!("{:?}", rotate_the_box(box_grid));
}
